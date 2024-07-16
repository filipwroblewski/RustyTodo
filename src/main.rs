use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
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
    HttpResponse::Created().json(new_task)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_list: TaskList = Arc::new(Mutex::new(Vec::new()));

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
