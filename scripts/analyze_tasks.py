import requests

def fetch_tasks():
    response = requests.get('http://127.0.0.1:8080/tasks')
    if response.status_code == 200:
        tasks = response.json()
        return tasks
    else:
        print("Failed to fetch tasks")
        return []

def analyze_tasks(tasks):
    completed: int = len([task for task in tasks if task['completed']])
    pending: int = len(tasks) - completed
    print(f"Completed tasks: {completed}")
    print(f"Pending tasks: {pending}")

if __name__ == "__main__":
    tasks = fetch_tasks()
    analyze_tasks(tasks)
