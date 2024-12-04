# Subscription Service

Welcome to the **Subscription Service** repository! This guide will help you set up the project environment and complete the assigned tasks related to implementing subscription functionalities.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup Instructions](#setup-instructions)
- [Assignment](#assignment)
    - [Part 1: Implement `list_subscriptions`](#part-1-implement-list_subscriptions)
    - [Part 2: Add Email Filter](#part-2-add-email-filter)

## Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust** (version 1.60 or later)
    - **Installation:** [Rust Official Website](https://www.rust-lang.org/tools/install)

2. **Cargo** (comes bundled with Rust)
    - **Verification:** Run `cargo --version` in your terminal.

3. **SQLite** (version 3.0 or later)
    - **Installation:**
        - **Ubuntu/Debian:**
          ```bash
          sudo apt update
          sudo apt install sqlite3 libsqlite3-dev
          ```
        - **macOS (using Homebrew):**
          ```bash
          brew install sqlite3
          ```
        - **Windows:**
          Download the precompiled binaries from the [SQLite Download Page](https://www.sqlite.org/download.html) and follow the installation instructions.

4. **Git** (for cloning the repository)
    - **Installation:** [Git Official Website](https://git-scm.com/downloads)

## Setup Instructions

### 1. Clone the Repository

Start by cloning the repository to your local machine:

```bash
git clone https://github.com/your-username/subscription-service.git
cd subscription-service
```

### 2. Start the Server

With the dependencies installed and the database initialized, you can now start the gRPC server:

```bash
cargo run
```

You should see logs indicating that the server has started and is listening on the specified port.

## Assignment

Your task involves enhancing the Subscription Service by implementing and extending the `list_subscriptions` functionality.

### Part 1: Implement `list_subscriptions`

**Objective:** Finish implementing the `list_subscriptions` function to retrieve all subscriptions from the database and test it using Postman.

**Steps:**

1. **Locate the Function:**
    - Open the file `src/handlers/api.rs`
    - Find the `list_subscriptions` function, which is currently unimplemented (`todo!()`)

2. **Implement Database Retrieval:**
    - Use the appropriate database queries to fetch all email entries from the subscriptions table
    - Populate the `ListSubscriptionsResponse` with the retrieved emails

3. **Test Using Postman:**
    - Setup Postman for gRPC
    - Create a new request
    - Set the request type to gRPC
    - Enter the server address (e.g., `localhost:50051`)
    - Invoke `ListSubscriptions`
    - Send the request and verify that all subscriptions are returned correctly

**Deliverables:**
- A working `list_subscriptions` function that retrieves and returns all subscribed emails
- Confirmation of functionality through successful Postman tests

### Part 2: Add Email Filter

**Objective:** Enhance the `list_subscriptions` functionality by adding the ability to filter subscriptions based on a provided email substring.

**Steps:**

1. **Update the Proto File:**
    - Open the `.proto` file defining the `ListSubscriptionsRequest`
    - Add an optional filter field to allow clients to specify a substring for filtering emails

2. **Regenerate Rust Code:**
    - After modifying the proto file, regenerate the Rust code using `tonic-build` or your preferred method

3. **Modify the `list_subscriptions` Function:**
    - Update the function to accept the filter parameter
    - Adjust the database query to include a condition that filters emails containing the specified substring

4. **Test the Filter:**
    - Using Postman:
        - Modify the `ListSubscriptions` request to include a filter value
        - Send the request and verify that only emails containing the specified substring are returned
    - **Edge Cases:**
        - Test with filters that match multiple, single, or no emails to ensure robustness

**Deliverables:**
- An updated `ListSubscriptionsRequest` that includes an optional filter field
- Enhanced `list_subscriptions` function capable of filtering results based on the provided email substring
- Successful tests demonstrating the filtering functionality