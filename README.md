# Harbor

> An educational HTTP server written in Rust.

Harbor is an open source project created with a simple purpose:

**to understand how web servers work by building one from scratch.**

Rather than focusing on performance, benchmarks or production-ready features, Harbor focuses on clarity, simplicity and learning.

Every major component will be implemented incrementally, allowing both the author and anyone following the project to understand what happens behind the abstractions commonly provided by modern web frameworks.

---

## Philosophy

Harbor is guided by a few simple principles:

* Build to understand.
* Keep the code readable.
* Prefer simplicity over cleverness.
* Add complexity only when necessary.
* Avoid external dependencies whenever it supports the educational purpose.
* Document architectural decisions throughout the project.
* Learn before optimizing.

Harbor is intended to be a project that can be read, explored and studied.

---

## Project Goals

The long-term vision includes:

* Build an HTTP server from scratch using Rust.
* Serve static files.
* Support dynamic content.
* Become the foundation for **Pilot**, a lightweight web framework.
* Eventually power a real-world blog built entirely on this ecosystem.

Harbor is the first component of a larger educational ecosystem composed of:

* **Harbor** — HTTP Server
* **Pilot** — Web Framework
* **Ledger** — Database
* **Devlog** — Engineering Journal

The ultimate goal is to publish and maintain a real blog powered exclusively by these technologies.

---

## Current Status

This project is currently in its earliest stage.

The initial focus is to understand Rust fundamentals and build the server incrementally, one milestone at a time.

No shortcuts.

No hidden magic.

Only software built step by step.

---

## Roadmap

The planned evolution is approximately:

* [ ] Initialize the Rust project.
* [ ] TCP listener.
* [ ] Accept incoming connections.
* [ ] Parse HTTP requests.
* [ ] Generate HTTP responses.
* [ ] Serve static files.
* [ ] Basic routing.
* [ ] Configuration system.
* [ ] Integration with Pilot.
* [ ] Power a real-world blog.

The roadmap will evolve as the project matures.

---

## Development Philosophy

Harbor is not developed as a race.

Every feature exists to answer a question.

Every commit should teach something.

The goal is not simply to build software.

The goal is to become a better software engineer by understanding every layer involved in building a web server.

---

## Development Journal

The complete engineering journey is documented in the companion **Devlog** repository, where architectural decisions, experiments, lessons learned and project milestones are recorded throughout the development process.

---

## License

This project will be released under the MIT License.
