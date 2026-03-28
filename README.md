# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
**1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or `trait` in Rust) in this BambangShop case, or a single Model `struct` is enough?**

First, I explain first question, which is: do we need a trait (interface) for Subscriber?

1. If your "subscriber" list has multiple independent behaviors. Use a trait:
- For example: `trait Subscriber { fn update(&self, ...); }`
- You can use polymorphism like this: (`Vec<Box<dyn Subscriber>>`)
- or you can easily to add new kinds of subscribers later (email, push, logs)

2. But if only one concrete observer type, which is a single model `struct` exists and won't grow (or rarely change/grow).
A single model `struct` may be enough because it is:
- simpler
- no dynamic dispatch
- fewer abstractions
- good for small scope / lower complexity
**However, I must beware this: future change can make a plain struct harder to extend cleanly.**

**2. `id` in `Program` and `url` in `Subscriber` is intended to be unique. Explain based on your understanding, is using `Vec` (list) sufficient or using `DashMap` (map/dictionary) like we currently use is necessary for this case?**

The answer is `DashMap` is necessary and the right choice because:

1. **Performance**: HTTP notification endpoints will frequently need to lookup subscribers by `url` or programs by `id`. O(1) hash map operations scale much better than O(n) vector searches.

2. **Uniqueness Enforcement**: HashMap keys inherently prevent duplicates, aligning with the "intended to be unique" requirement without additional validation code.

3. **Concurrency Safety**: Rocket's async nature means multiple threads could access subscriber/program data simultaneously. DashMap provides lock-free concurrent access, preventing race conditions that Vec would be vulnerable to.

4. **Future-Proofing**: As the system grows (more subscribers/programs), the performance difference becomes critical.

**3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (`SUBSCRIBERS`) static variable, we used the `DashMap` external library for *thread safe* `HashMap`. Explain based on your understanding of design patterns, do we still need `DashMap` or we can implement Singleton pattern instead?**

Based on the code in `subscriber.rs`, the `SUBSCRIBERS` static variable is a `DashMap<String, DashMap<String, Subscriber>>` initialized with `lazy_static`!. This provides a thread-safe, concurrent hash map for storing subscribers grouped by product type.

The Singleton pattern ensures that only one instance of a class or struct exists throughout the program's lifetime. However, it does not inherently address thread safety for data access in a concurrent environment. In this Rust project, which uses the Rocket web framework (an async, multi-threaded runtime), multiple threads may simultaneously read from or write to `SUBSCRIBERS` (e.g., during subscription/unsubscription or notification operations).

Why `DashMap` is necessary: `DashMap` is a lock-free, thread-safe hash map that allows safe concurrent access without manual synchronization (e.g., no need for `Mutex` or `RwLock`). Rust's borrow checker and ownership rules enforce thread safety at compile time, and `DashMap1` aligns with this by providing atomic operations. Replacing it with a standard HashMap (even in a Singleton) would not compile due to potential data races in multi-threaded contexts.

Singleton vs. `DashMap`: The Singleton pattern could be implemented using `once_cell::sync::Lazy` or similar for lazy initialization of a single instance. However, the data structure inside the Singleton still needs to be thread-safe. A Singleton wrapping a non-thread-safe HashMap would fail Rust's compiler checks. Thus, Singleton alone does not eliminate the need for DashMap; it merely ensures one global instance, while DashMap handles the concurrency.

In summary, DashMap is required for thread safety, and the Singleton pattern is already partially achieved via `lazy_static`!. Attempting a pure Singleton without a thread-safe map would violate Rust's constraints (memory safety, ownership, and concurrency to prevent data races at compile time through its ownership and borrowing system via mutiple threads simultaneously read/write to the same map).


#### Reflection Publisher-2
**1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?**

Separating Service and Repository from the Model is essential for building scalable, maintainable, and testable software. Here's why:

**1. Separation of Concerns (SoC)**
   - **Model**: Represents data structure and validation logic only (what a product is)
   - **Repository**: Handles all data persistence operations (how to store/retrieve products)
   - **Service**: Contains business logic and orchestration (what operations to perform on products)
   
   This separation ensures each layer has a single, well-defined responsibility, making the codebase easier to understand and modify.

**2. Single Responsibility Principle (SRP)**
   - A Model class that handles data storage, retrieval, AND business logic violates SRP by having multiple reasons to change
   - Repository changes only when database strategy changes (SQL, NoSQL, caching, etc.)
   - Service changes only when business rules change
   - Model changes only when data structure changes
   
   This independence means changes in one area don't cascade to others.

**3. Testability**
   - Model can be tested with unit tests in isolation (pure data structures)
   - Repository can be tested with mock databases or in-memory databases
   - Service can be tested by mocking the Repository layer, allowing us to test business logic without hitting actual databases
   
   In the original MVC Model, all three concerns are mixed, making it nearly impossible to test business logic independently from database operations.

**4. Code Reusability**
   - The same Repository can be used by multiple Services (e.g., `ProductRepository` used by both `ProductService` and `NotificationService`)
   - The same Service can support multiple Controllers or API endpoints
   - Business logic in Services can be reused across different presentation layers (web, CLI, API)
   
   Without separation, business logic becomes tied to specific data access patterns or presentation layers.

**5. Flexibility and Maintainability**
   - You can swap Repository implementations without changing Service or Controller (e.g., migrate from in-memory `DashMap` to PostgreSQL)
   - You can update business rules in Service without touching Models or data access code
   - New features often require only new Service methods, not Model changes
   
   This makes the system more maintainable as requirements evolve.

**6. Scalability and Performance**
   - Repository layer allows for optimization strategies (caching, lazy loading, batch operations) without affecting business logic
   - Service layer can coordinate between multiple Repositories efficiently
   - In the original MVC Model, optimizing data access would require modifying business logic code
   
   This layering allows performance improvements at the data access level independently.

**Real Example from BambangShop:**
- If business logic changes (e.g., "only notify subscribers every 5 minutes"), you only modify `NotificationService`
- If you want to migrate from `DashMap` to a real database, you only change `SubscriberRepository`
- If you want to add email notifications alongside HTTP notifications, you only add to the Service layer without touching Repository or Model

Without separation, all three would be tightly coupled, and any change could introduce bugs or require massive refactoring. The three-layer separation is what makes modern software architecture robust, testable, and adaptable to change without damaging the application or project because of loosely coupled.

**2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (`Program`, `Subscriber`, `Notification`) affect the code complexity for each model?**

If we only used Models without separating Service and Repository layers, the code complexity would skyrocket due to tight coupling and circular dependencies between models. Here's how:

**Model Interactions and Coupling:**

1. **Product Model** would need to:
   - Store data (name, description, price)
   - Access and manage itself in a static/global storage (like `PRODUCTS`)
   - Directly access the `SUBSCRIBERS` collection to notify them
   - Directly make HTTP POST requests to subscriber endpoints
   - Handle database operations (CRUD)
   
2. **Subscriber Model** would need to:
   - Store subscription data (id, url, product_type)
   - Access and manage itself in static/global storage (like `SUBSCRIBERS`)
   - Receive notifications from `Product` (bidirectional dependency)
   - Send HTTP POST requests independently
   - Know how to serialize/deserialize from HTTP requests

3. **Notification Model** would need to:
   - Track notification history
   - Access both `Product` and `Subscriber` models
   - Orchestrate the notification process itself
   - Know about HTTP request formatting and sending
   - Manage timing and retry logic

**Concrete Complexity Issues:**

**Circular Dependencies:**
```
Product → needs to notify → Subscriber
Subscriber → needs to receive from → Product
Notification → needs both Product and Subscriber
```
This creates circular reference problems. In Rust, this would cause compilation errors or require complex lifetime management across multiple models.

**Explosion of Responsibilities:**
- Each model becomes a "God Object" (all-knowing object) with too many responsibilities
- Product model: ~200+ lines (data + storage + notification logic + HTTP calls + database sync)
- Subscriber model: ~150+ lines (data + storage + HTTP receiving + listening logic)
- Notification model: ~100+ lines (orchestration + history + retry logic)
- **Total: 450+ lines of tightly coupled code**

With current separation:
- Model (data only): ~20-30 lines
- Repository (persistence): ~50-80 lines
- Service (business logic): ~60-100 lines
- **Total: Still ~180 lines but cleanly separated and reusable**

**Testing Nightmare:**
- Cannot make unit test Product creation without triggering notification
- Cannot make unit test Subscriber registration without HTTP dependencies
- Cannot make unit test Notification logic without real database and HTTP setup
- Must mock everything at once, making tests brittle and slow

**Data Consistency Issues:**
- If Product notifies Subscriber, and the HTTP call fails, how does Product know?
- Does Product retry? Does Subscriber retry? Who manages state?
- With Models handling everything, this logic gets lost in the code
- Service layer makes this explicit: `NotificationService::notify_subscribers()`

**Change Propagation Nightmare:**
- Want to add email notifications? Must modify 3 models
- Want to change notification format? Must touch Notification, Product, and Subscriber
- Want to add a new product type? Every model that touches products must change
- With separation: Only add `EmailNotificationService`, no changes to models

**Scalability Problem:**
- If we later need to notify 10,000 or even 100 thousand subscribers, who manages the async/concurrent notification?
- Models can not efficiently handle batch operations—Service layer is designed for this
- Performance optimizations (caching, batching, rate limiting) must live somewhere—without Service, they would clutter Models

**Memory and Reference Issues in Rust:**
- Rust's borrow checker would struggle with circular references between models
- Each model would need `Arc<Mutex<...>>` or `RefCell<...>` to manage shared mutable state
- This hides ownership issues and makes the code less idiomatic
- Service layer avoids this by centralizing data access through Repository

**Real Example - Just Adding Notification:**
If Product model must notify Subscribers, then a method must become 50 or more lines, mixing persistence, business logic, HTTP, error handling.

**Conclusion:**
Using only Models would result in:
- **High Complexity**: Each model intersects with 2-3 others, creating dense interdependencies
- **Poor Maintainability**: A single change (like "notify every 5 minutes instead of immediately") requires modifying multiple models
- **No Reusability**: Business logic trapped inside models can't be used by new Controllers or features
- **Impossible to Test Independently**: Can not test one model without testing all its dependencies
- **Violates SOLID Principles**: Especially Single Responsibility and Open/Closed Principle

The Service and Repository separation exists precisely to prevent this collapse into spaghetti code. Models stay simple, focused only on data representation.

**3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.**

I explained first why Postman is useful for this project.
- API-first workflow: Postman lets you send GET, POST, PUT, DELETE easily, exactly like your Rocket endpoints.
- Fast iteration: hit /product, /subscriber, /notification quickly after code changes without manual frontend.
- Debugging: inspect request/response headers, JSON body, status codes (200, 201, 400, 404) in one place.
- Environment variables: store APP_INSTANCE_ROOT_URL for local vs deployed tests.
- Collections: you already have a Postman Collection link usable by team members consistently.

# **Features I’d use in BambangShop**
**1. Collections + folders**
- Keep "Product API", "Subscriber API", "Notification API" organized.
**2. Pre-request scripts**
- Set token/URL and dynamic IDs (e.g., created product id for next request).
**3. Tests scripts**
- Automate assertions `(pm.expect(response.code).to.equal(200))`.
**4. Monitor**
- Schedule health checks for /product//notify in CI-like cadence.
**5. Mock servers**
- Simulate external subscribers receiving notifications (without real Rocket instance for each subscriber).


#### Reflection Publisher-3
