# RustyTodo

Simple task management API built with Actix-web in Rust. 
Tasks are stored in memory and can be retrieved or created via HTTP requests.

## Cloning the Repository

To clone the repository, run the following command:

```
git clone <repository_url>
cd <repository_name>
```

## Project Setup

### Rust Setup

Ensure you have Rust and Cargo installed. If not, you can install them from [Rust's official website](https://www.rust-lang.org/).

## Python Setup

1. Create a virtual environment in the project directory:

    ```
    python -m venv venv
    ```

2. Activate the virtual environment:

    - On Windows:

        ```
        venv\Scripts\activate
        ```

    - On macOS and Linux:

        ```
        source venv/bin/activate
        ```

3. Install the required Python dependencies:

    ```
    pip install -r requirements.txt
    ```

## Running the Project

### Set Environment Variables

Create a `.env` file in the root of your project directory and add the necessary environment variables. Example for SQLite:

```env
DATABASE_URL=sqlite://database.db
```

_**Note**: Make sure that the database file path is correct and accessible_

### Rust

1. Build and run the Rust application:

    ```
    cargo run
    ```

### Python

1. Make sure the virtual environment is activated.
2. Run Python apps:

    ```
    python scripts/manage_tasks.py
    python scripts/analyze_tasks.py
    ```

## API Usage

The server will be running at [http://127.0.0.1:8080](http://127.0.0.1:8080). You can interact with the API using tools like Postman or CURL, or by using the provided Python scripts.