# pzn
(*Project Zonder Naam*)

## Usage:

```bash
$ python gen.py
$ cd pzn
$ cargo run
```


## The premise

Over the years, "the browser" has become a very rich platform for
applications that can run cross-platform with minimal effort. However,
browser's have also become increasingly complex, so your favourite
chat app uses 6 process and nearly 400 MiB of RAM, which is insane for
an app whose main purpose is to relay short text messages between users.

WebAssembly (WASM) provides a way to represent fast, safe, and
cross-platform code. We can leverage it to get out of this mess.

What do we really need when developing a graphical application? What
if we had a canvas and an API to draw text and simple shapes? And let's
add GL to that. This is already a pretty rich and powerful API, on top
of which more advanced things can be built.


## The idea

The idea is to define an API for graphics that can be used to
build a wide variety of applications. This standard can be implemented
by various runtimes, e.g. inside the browser or in Rust. 

The API is asynchronous. It can be used via function calls from within
the same process, or via commands send over pipes and/or sockets. (Note
the similarity to the terminal and the nature of the GL API). By
allowing the communication to be done from another process, the coupling
is low, making it much easier for applications in many different
languages to target the same runtime.

Some example use-cases:

* A JavaScript app running in the same browser session.
* A Python app running on a server, communicating via a websocket.
* A Rust desktop application.
* A Julia process running a lightweight local GUI.
* Standalone interactive scientific visualization that can be shown in
  the browser or as a ~1 MiB desktop app.
* I'm pretty sure we can do an awesome chat app in <20 MiB.

An important aspect is the use of WASM to communicate “behavior” to the
graphics endpoint. This is similar to how JavaScript is used to script
a web page, and how shaders are used to render data in GL. So the
application does not use the drawing API's directly, but sends WASM
modules to the runtime. The runtime runs these WASM modules, and
provides them with the actual drawing API's.

The application can also push/update data to the runtime, and invoke
function calls in the WASM modules (e.g. to set local state). The WASM
modules export a function called “draw”, which is called by the runtime
at the right moment, which is where the drawing API calls will be done,
using local state and data. Events (e.g. user interaction) can be
handled by the WASM modules or communicated to the application.

It’s important that the supported drawing API’s are a subset of
what browsers support, so that this technology can used inside a
browser. At the same time, we want it to be feasible to implement them
in e.g. Rust, and simple enough that resulting applications are
reasonably sized. So a DOM is out of the question.

Consequently, the requirements of a runtime are:

* Being able to bind and execute WASM modules. This is probably the toughest
  requirement right now. But it won't a few years from now (e.g. Wasmer).
* Implement a canvas drawing API (e.g. via SDL2, or QPainter).
* Implement a GL drawing API (e.g. via SDL2, GLFW, or QGLWidget).
* ... additional requirements may be added, but we aim "light".


## The plan

There are many loose ends and things to work out. Let's start by demonstrating
the idea in some forms:

* Implement a proof of concept runtime in Rust.
* Implement a typed-Python to WASM compiler (with support for strings).
* Implement a runtime in the browser using JavaScript modules instead of WASM,
  because we can already create these with PScript. This allows demonstrating
  advanced usage from Python quickly while the point above is in progress.
