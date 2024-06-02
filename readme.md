
# BerryLib

BerryLib is a Rust-based web application framework leveraging Actix Web for building high-performance web services. This project includes modules for controllers, models, routes, services, and middleware, with a focus on authentication and database connectivity.

## Table of Contents
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Features](#features)
- [Environment Variables](#environment-variables)
- [Running the Application](#running-the-application)
- [Endpoints](#endpoints)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

### Prerequisites
- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- PostgreSQL: [Install PostgreSQL](https://www.postgresql.org/download/)

### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/berrylib.git
   cd berrylib
   ```

2. Set up environment variables:
   Create a `.env` file in the root directory with the following contents:
   ```env
   DATABASE_URL=your_postgres_database_url
   PORT=8080
   SECRET_KEY=your_secret_key
   ```

3. Install dependencies and build the project:
   ```sh
   cargo build
   ```

## Project Structure
```
berrylib/
├── src/
│   ├── controllers/
│   ├── middleware/
│   │   └── auth_middleware.rs
│   ├── models/
│   ├── routes/
│   ├── services/
│   ├── main.rs
│   └── ...
├── .env
├── Cargo.toml
└── README.md
```

## Features
- **Actix Web Integration**: Leverages Actix Web for high-performance web service development.
- **Authentication Middleware**: Custom middleware for handling JWT authentication.
- **Database Connectivity**: Utilizes SQLx for PostgreSQL database operations.
- **Environment Configuration**: Manages configuration using dotenv.

## Environment Variables
The application uses the following environment variables, which should be defined in a `.env` file:

- `DATABASE_URL`: URL of the PostgreSQL database.
- `PORT`: Port on which the server will run.
- `SECRET_KEY`: Secret key for JWT authentication.

## Running the Application
1. Start the PostgreSQL database.

2. Run the application:
   ```sh
   cargo run
   ```

3. The server will start on the specified port (default is 8080).

## Endpoints
### Authentication
- `POST /auth/login`: Authenticate a user and return a JWT.
- `POST /auth/register`: Register a new user.

### Other Routes
- Define other routes in the `routes` module.

## Contributing
Contributions are welcome! Please fork the repository and create a pull request with your changes.

### Steps to Contribute
1. Fork the repository.
2. Create a new branch with your feature or bugfix.
3. Commit your changes.
4. Push to your branch and submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
