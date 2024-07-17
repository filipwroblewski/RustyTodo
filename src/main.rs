use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTask {
    title: String,
}

type TaskList = Arc<Mutex<Vec<Task>>>;

const TASKS_FILE: &str = "tasks.json";

fn read_tasks_from_file() -> io::Result<Vec<Task>> {
    let file = File::open(TASKS_FILE).or_else(|_| File::create(TASKS_FILE))?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn write_tasks_to_file(tasks: &Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TASKS_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}

async fn get_tasks(tasks: web::Data<TaskList>) -> impl Responder {
    let task_list = tasks.lock().unwrap();
    HttpResponse::Ok().json(&*task_list)
}

async fn create_task(task: web::Json<CreateTask>, tasks: web::Data<TaskList>) -> impl Responder {
    let mut task_list = tasks.lock().unwrap();
    let new_task = Task {
        id: task_list.len() as i32 + 1,
        title: task.title.clone(),
        completed: false,
    };
    task_list.push(new_task.clone());
    write_tasks_to_file(&task_list).unwrap();
    HttpResponse::Created().json(new_task)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_tasks = read_tasks_from_file().unwrap();
    let task_list: TaskList = Arc::new(Mutex::new(initial_tasks));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_list.clone()))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
