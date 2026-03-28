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
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

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

According to me, separating Service and Repository from the Model is essential for building scalable, maintainable, and testable software. Here's why:

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

**1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?**

In this BambangShop tutorial case, we use the **Push model** of the Observer Pattern.

Evidence from the implementation:
- In `ProductService`, when a product is created or deleted, the service actively calls `notify_subscribers()` which sends notifications to all interested subscribers
- In `NotificationService::notify()`, the publisher iterates through all subscribers and pushes notifications by making HTTP POST requests directly to each subscriber's endpoint
- The notification is triggered by product operations (create/delete), not by subscribers requesting updates
- Subscribers are passive—they receive HTTP POST requests and don't initiate polling or pulling

The flow is:
```
Product Created → ProductService → NotificationService::notify() → Iterates subscribers → HTTP POST to each subscriber's URL
```

This is characteristic of the Push model: **the publisher (BambangShop) actively sends data to all subscribed clients**.

**2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (if you answer question number 1 with Push, then imagine if we used Pull)**

If we switched to a **Pull model**, subscribers would periodically retrieve updates from the publisher instead of receiving pushed notifications. Here's how that would look and its implications:

**Advantages of Pull Model:**

1. **Loose Coupling**: Subscribers don't need to register their HTTP endpoints. The publisher doesn't need a list of subscriber URLs.
   - Subscribers can be dynamically added/removed without notifying the publisher
   - Publisher doesn't care about subscriber availability

2. **Subscriber Autonomy**: Each subscriber decides when to fetch and how often
   - A subscriber can prioritize which products to check based on its own schedule
   - Subscribers control their own polling frequency based on needs

3. **Reduced Publisher Load Spikes**: The publisher doesn't need to send notifications to many subscribers simultaneously
   - No "thundering herd" problem during large product updates
   - Smoother, more distributed load across time

4. **Subscriber Resilience**: If a subscriber is temporarily offline, it still gets updates when it comes back online
   - Pull model: subscriber just fetches missed updates
   - Push model: subscriber misses notifications while offline

5. **Simpler Error Handling for Publisher**: Publisher doesn't need to retry failed deliveries or manage subscriber endpoints
   - No need to track which subscribers failed to receive notifications
   - Publisher doesn't need to validate or maintain subscriber URLs

**Disadvantages of Pull Model:**

1. **Latency & Staleness**: Updates are not real-time; there's always a delay
   - If a subscriber polls every 5 minutes, they could be 5 minutes behind
   - For a real-time sales/promotion system, this is unacceptable
   - "Your product was just deleted, but subscriber doesn't know for 5 minutes"

2. **Wasted Network Bandwidth**: Subscribers continuously poll even when there are no updates
   - 100 subscribers polling every 10 seconds = 600 requests per minute
   - Most polls return "no new updates," wasting bandwidth and server resources
   - In contrast, Push sends only when there's actual news

3. **Higher Publisher Load**: Constant polling creates sustained high load
   - Must maintain an endpoint that checks for updates by product type for each subscriber
   - Queries the database repeatedly over time instead of once on an actual change
   - Horizontal scaling becomes harder

4. **Complex Subscriber Logic**: Subscribers need to manage polling state
   - Must track last-seen update timestamp to know what's new
   - Must implement retry logic if a poll fails
   - Must implement duplicate detection (what if update was fetched twice?)

5. **Difficult to Guarantee Delivery**: No hard guarantee all subscribers receive an update
   - A subscriber might be offline/broken and never pull the update
   - Need mechanisms to mark old updates as "must be fetched"

6. **Subscriber Registration Still Needed**: Paradoxically, you still need a way for subscribers to register their interests
   - Subscribers must somehow notify the publisher: "I'm interested in product type X"
   - You haven't eliminated the registration; you've just separated notification from subscription

7. **Resource Inefficiency**: Especially problematic at scale
   - In BambangShop with 1000 subscribers polling every 5 seconds = 200 requests/second just checking for updates
   - Push model with 10 products/minute updates = 10,000 notifications/minute = ~167 requests/second, but only when necessary

**Pull Model Implementation in BambangShop would look like:**
```rust
// Instead of: /notification/subscriber (subscribe)
// New endpoint: /notification/updates?product_type=food&since=2024-01-01T10:00:00Z

// Subscribers poll this endpoint repeatedly:
GET /api/v1/notification/updates?product_type=food&since=2024-01-01T10:00:00Z
// Response: [Product { id, event_type, timestamp }]
```

**Verdict for BambangShop**: The **Push model is clearly superior** for this use case because:
- Products are created/deleted infrequently (low update frequency)
- Subscribers need near-real-time notification (notifications are about commerce events)
- The number of subscribers is manageable (likely hundreds, not millions)
- The tutorial emphasizes immediate notification, not eventual consistency

The Pull model would make sense for scenarios like:
- System monitoring (check every minute if any alerts exist)
- News/content feeds (subscribers don't need real-time updates)
- Low-frequency, high-latency-tolerant systems

**3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.**

If we removed multi-threading from the notification process and made it synchronous, the application would suffer severe performance and responsiveness issues. Here's what would happen:

**Scenario: User creates a new Product with 100 registered subscribers**

**With Multi-threading (Current Implementation):**
```
User clicks "Create Product"
  → Controller receives request
  → Service creates product in repository (fast, < 1ms)
  → Service spawns thread to notify subscribers
  → Controller immediately returns 200 OK to user (< 10ms)
  → In background: 100 HTTP POST requests sent to subscribers (takes ~5-30 seconds)
  → User sees "Product created" immediately and continues browsing
```

**Without Multi-threading (Synchronous Blocking):**
```
User clicks "Create Product" → Controller receives request → Service creates product in repository (< 1ms) 
  → Service synchronously notifies each subscriber:
    ├─ Subscriber 1: HTTP POST request (2 seconds)
    ├─ Subscriber 2: HTTP POST request (2 seconds)
    ├─ Subscriber 3: HTTP POST request (2 seconds)
    ├─ ... (repeat 100 times)
    └─ Subscriber 100: HTTP POST request (2 seconds)
  → Total: ~200 seconds (3+ minutes!) → Only THEN does controller return 200 OK to user → User stares at loading screen for 3+ minutes
```

**Concrete Problems That Occur:**

1. **Blocking Request Threads**: 
   - Rocket runs on async threads (Tokio runtime)
   - Each HTTP request uses one async task
   - If that task blocks on network I/O (HTTP POST to subscribers), the thread can't process other requests
   - With 10 concurrent product creations × 100 subscribers each, you need 1000+ threads
   - The Tokio runtime would be starved

2. **Timeout Failures**:
   - HTTP requests have timeouts (typically 30 seconds)
   - If a subscriber's server is slow or unresponsive, the entire product creation hangs
   - One slow subscriber delays all 100 remaining subscribers
   - Request timeout might trigger before finishing all notifications

3. **Resource Exhaustion**:
   ```
   10 users create products simultaneously
   × 100 subscribers per product
   = 1000 concurrent HTTP operations
   ```
   - System runs out of file descriptors/network sockets
   - New HTTP connections fail: "Too many open files"
   - Server becomes unresponsive

4. **Poor User Experience**:
   - User creates a product and waits 3+ minutes for response
   - User creates 10 products and waits 30 minutes total
   - Browser/client times out before notification completes
   - Users think the server is broken

5. **Cascade Failures**:
   - If one subscriber is offline or extremely slow, it delays everyone
   - One bad subscriber can make the entire system appear frozen
   - No way to skip a slow subscriber and continue

6. **No Parallelism**:
   - If notifying 100 subscribers takes ~2 seconds each:
     - Multi-threaded: ~10-30 seconds total (parallel to all)
     - Single-threaded: ~200 seconds total (sequential)
   - A 10-20× performance penalty

7. **Rocket's Async Model Breakdown**:
   - Rocket is designed for async/non-blocking operations
   - Forcing synchronous blocking violates the async paradigm
   - The event loop (Tokio) can't handle blocking operations efficiently
   - It's like trying to run a car in reverse on the highway—technically possible but very bad design

**Real-World Example**:
```rust
// WITHOUT multi-threading (WRONG):
#[post("/products", format = "json", data = "<form>")]
async fn create_product(form: Json<NewProductRequest>) -> Json<Product> {
    let product = ProductService::create(&form);
    
    // These HTTP requests block the async task!
    for subscriber in get_all_subscribers() {
        let response = blocking_http_post(&subscriber.url, &notification)?;
        // Each line waits 2-5 seconds
        // Total request duration: 200+ seconds
    }
    
    Json(product)  // User finally gets response after 3+ minutes
}

// WITH multi-threading (CORRECT):
#[post("/products", format = "json", data = "<form>")]
async fn create_product(form: Json<NewProductRequest>) -> Json<Product> {
    let product = ProductService::create(&form);
    
    // Spawn async task to notify in background
    tokio::spawn(async move {
        NotificationService::notify_all_subscribers(product_id).await;
        // Happens in background while user already has response
    });
    
    Json(product)  // User gets response immediately (< 100ms)
}
```

**Solution in Current Code**:
The current implementation correctly uses async/threading because:
- `tokio::spawn()` spawns independent async tasks for notifications
- Each subscriber notification doesn't block the HTTP response
- Multiple subscribers are notified in parallel
- The main request returns immediately

**Analogy**:
- **Multi-threaded (correct)**: Like a restaurant where the chef starts cooking your food and you sit at the table while it prepares. You don't have to watch each step; you relax and eventually get your meal.
- **Synchronous (wrong)**: Like a drive-thru where the worker makes your entire order one item at a time while you wait in the car. If they're making 100 cars' orders first, you wait hours.

**Conclusion**: Without multi-threading, the notification process would make the application completely unusable for any real-world scenario with multiple subscribers. The response times would increase from milliseconds to minutes, causing user frustration and potential timeouts. This demonstrates why async/multi-threaded notification is not just a performance optimization—it's an architectural necessity for responsive web applications.
