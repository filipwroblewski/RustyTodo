import requests
import json

base_url = "http://127.0.0.1:8080"

def create_task(title):
    url: str = f"{base_url}/tasks"
    headers: dict[str, str] = {
        "Content-Type": "application/json"
    }
    data = {
        "title": title
    }
    response = requests.post(url, headers=headers, data=json.dumps(data))
    
    if response.status_code == 201:
        print(f"Task '{title}' created successfully!")
    else:
        print(f"Failed to create task: {response.status_code}")
        print(response.text)

def get_tasks():
    url: str = f"{base_url}/tasks"
    response = requests.get(url)
    
    if response.status_code == 200:
        tasks = response.json()
        print("Tasks:")
        for task in tasks:
            print(f"ID: {task['id']}, Title: {task['title']}, Completed: {task['completed']}")
    else:
        print(f"Failed to retrieve tasks: {response.status_code}")
        print(response.text)

if __name__ == "__main__":
    create_task("Learn Rust")
    get_tasks()
