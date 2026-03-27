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
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
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

#### Reflection Publisher-3
