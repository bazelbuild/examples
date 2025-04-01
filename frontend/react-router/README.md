# React Router

This example demonstrates how to use [React Router](https://reactrouter.com/) with Bazel. It uses React Router v7, into which [Remix](https://remix.run/) has been merged ([blog post](https://remix.run/blog/merging-remix-and-react-router)).

The app code for this example is the default template project, generated from the [setup instructions](https://reactrouter.com/start/framework/installation).

# TODO

- Use the output from `//react-router:typegen` to resolve IDE type errors in `app/root.tsx` and `/app/routes/home.tsx` and fix `bazel test //react-router/app:app_typecheck`.
- Consider alternatives to vendoring the `entry.client.tsx` and `entry.server.tsx` files, which are supposed to be optional.