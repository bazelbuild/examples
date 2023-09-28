This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app) and then Bazelified.

## Initial setup

When you run `create-react-app`, it installs a build system called `react-scripts`. As a first step into Bazel, this example simply wraps the existing build system. This guarantees compatibility with your current code, and if your objective is just to include a frontend app into a bigger full-stack Bazel build, this might be the final step in the migration. However it will run `react-scripts` as a single Bazel action, which means that you gain no incrementality benefit. So we expect for most applications this is just a first step.

## Available Scripts

Just like with stock create-react-app, we have the same developer workflow. In the project directory, you can run:

### `yarn start`

Runs the app in the development mode.<br />
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.<br />
You will also see any lint errors in the console.

### `yarn test`

Launches the test runner in the interactive watch mode.<br />
Note that it restarts Jest on each change, so there's a large time penalty on each re-run.
This can be solved by making Jest ibazel-aware as we did with Karma.
See https://github.com/bazelbuild/rules_nodejs/issues/2028

### `yarn build`

Builds the app for production to the `build` folder.<br />
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br />
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

## Next steps

The next step beyond this example would be making the build more incremental and performant. The `react-scripts` build system would be split into multiple Bazel actions that could be independently run and cached by Bazel. This would allow for incremental builds, and would be the next step in the migration. We continue to transpile TS to JS using Babel, for example, but we do it in a build step before invoking Webpack, just using the Babel CLI.