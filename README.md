# Project Backend Documentation

## Overview
This backend system is built using [Actix Web](https://actix.rs/) and serves as the server-side component for [React Job Portal](https://github.com/LeafDevs/React-Job-Portal).

## Tech Stack
- Language: Rust
- Framework: Actix-Web
- Database: SQLite - Testing | MariaDB - Production
- Authentication: Custom

## Features
- User Authentication and Authorization
- Easily Hook onto Other Sites
- Encrypted and Securely Stored user information.

## Setup Instructions

### Prerequisites
- Rust Installed
- 2GB Ram
- Unix Terminal (WSL or Linux/MacOS)
- A Database Server (MySQL/MariaDB/Postgres)

### Installation
1. Clone the repository
   ```bash
   git clone https://github.com/LeafDevs/job-portal-backend
   ```

2. Set up environment variables
   ```
   Create a .env file with the following variables:
   - GOOGLE_CLIENT_SECRET=
   - GOOGLE_CLIENT_ID=
   ```

3. Start the server
   ```bash
   cargo run
   ```

## Error Handling
The API uses the following error codes:
- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 404: Not Found
- 500: Internal Server Error

## Security
- Rate Limiting
- Encrypted Data
- HTTPS via Let's Encrypt

## Testing
```bash
cargo test
```

## Compile
```bash
cargo build --release
```

## API Documentation
Detailed API documentation can be found at [https://api.lesbians.monster/](https://api.lesbians.monster/).

## License
This project is protected under

## Contact
Contact me via [Twitter](https://x.com/leaftopg)
