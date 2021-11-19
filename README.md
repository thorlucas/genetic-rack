
# Thor's React App Template

This is my minimal React application starter-kit consisting of the following stack:

- **React JS**
- **Typescript**
- **PostCSS** with **TailwindCSS**
- **Craco**

It features an example layout structure that keeps things tidy in your application.
Browse around the `src` folder and familiarize yourself with the layout. Several example
files are included, which demonstrate how you would use this layout.

### Table of Contents

- [Quick Start](#quick-start)
- [Aliases](#aliases)
- [Structure](#structure)
- [Style Guide](#style-guide)

## Quick Start

- Create a repository using this template by clicking "use template" at the top.
- Clone your new repo locally.
- Run `yarn` to install all dependencies.
- Run `yarn start` to launch the development server.
- Edit away! Any changes made are automatically rendered in the browser.

## Structure

Here is a pruned tree structure of this template:

<pre>
├── public
├── src
│   ├── adapters
│   ├── assets
│   ├── components
│   │   └── UI
│   ├── contexts
│   └── hooks
│ 
├── craco.config.js
├── tailwind.config.js
│ 
├── tsconfig.json
└── tsconfig.paths.json
</pre>

<details>
  <summary>Components</summary>

### Components

This folder will contain all of your ordinary components. Each component should be its
own folder with a root `index.ts` file. The index should default export the namesake
component. For example, `src/components/Foo/index.ts` should `default export Foo`. These
components can then be imported using the alias.

```typescript
import MyComponent from '@components/MyComponent';
```

Additionally, UI-type components should go in the UI subdirectory. These are things like
buttons or form elements. These are distinct from ordinary components for clarity. These
have their own alias, `@UI`.

```typescript
import Button from '@UI/Button';
```
</details>

<details>
  <summary>Hooks</summary>

### Hooks

This folder will contain all of your custom hooks. Each hook can either be its own file
or a folder with a root `index.ts`. The filename should be the name of
the hook function sans the 'use'. For example, the use `useWait` hook is found at
`wait.ts`. These should also `default export` the namesake hook.

```typescript
import useWait from '@hooks/wait';
```

Additionally, the `hooks` folder contains an `index.ts` itself. Here, important hooks
are re-exported so that they may be included using the `@hooks` alias alone.

```typescript
import { useWait } from '@hooks';
```
</details>

<details>
  <summary>Adapters</summary>

### Adapters

Adapters are used to wrap external logic. They might wrap the nitty-gritty details of interacting with an API, or they might wrap a library that's written in an ugly way. These can be implemented in any way you like and are very much just up to your use-case. You may just have a function that will return a promise with the result from an API call. Or you might have some class that represents a client connection to a more complex API.

The adapter folder has a root `index.ts` and adapters can be imported similarly to hooks:

```typescript
import fetchDocument from '@adapters/document';
// or
import { fetchDocument } from '@adapters';
```
</details>

<details>
  <summary>Assets</summary>

### Assets

This folder holds static assets such as images, icons, audio files… anything that you might want to import into a component. Thanks to React, if we have an image called `lightning.svg` in `assets`, then we can import this asset normally.

```typescript
import lightning from '@assets/lightning.svg';

const image = <img src={ lightning } alt="lightning" />
```
</details>

<details>
  <summary>Public</summary>

### Public

This is pretty self explanatory - it contains the `index.html` containing the root div where the React application is initially mounted. It also has the support files like `favicon.ico`, `robots.txt`, etc.
</details>

<details>
  <summary>Configuration Files</summary>
  
### Configuration Files

#### Craco

`craco.config.js` contains our Craco configuration. Craco is a configuration layer that allows for modification of the Create React App build system. Here you can add plugins that modify the building of the React application. You can view a list of Craco plugins [here](https://github.com/gsoft-inc/craco).

#### TailwindCSS

`tailwind.config.json` contains our TailwindCSS configuration. [TailwindCSS](https://tailwindcss.com/) is a CSS framework that provides a huge number of "utility classes". In contrast to Bootstrap, which gives us pre-built classes such as `btn` or `card`, TailwindCSS instead gives us only utilities that match one-to-one with CSS. For example, `mb-4` sets `margin-bottom: 1rem;`, `flex` sets `display: flex;`, and *et cetera*. TailwindCSS has [excellent documentation](https://tailwindcss.com/docs) which will get you familiarized with the utility classes quickly. [TailwindUI](https://tailwindui.com/) also provides some example UI components built with TailwindCSS.

#### Typescript

`tsconfig.json` provides the configuration for the Typescript compiler. You can change this to modify the behavior of the compiler, such as allowing certain language features or enabling certain lints.

`tsconfig.paths.json` is a special file that contains all of our path aliases. You can modify these to add or remove path aliases, which is a *must* as your project structure grows.
</details>

## Aliases

There are a number of path aliases defined in `tsconfig.paths.json`. The use of these aliases is heavily encouraged for readibility. You are also encouraged to add new aliases as your project structure grows.

*default aliases*
| Alias | Path |
|--|--|
| `@components/*` | `./src/components/*` |
| `@UI/*` | `./src/components/UI/*` |
| `@assets/*` | `./src/assets/*` |
| `@contexts` | `./src/contexts/index.ts` |
| `@contexts/*` | `./src/contexts/*` |
| `@hooks` | `./src/hooks/index.ts` |
| `@hooks/*` | `./src/hooks/*` |
| `@adapters` | `./src/adapters/index.ts` |
| `@adapters/*` | `./src/adapters/*` |


You may use them in your import statements:

```typescript
import Foo from '@components/Foo';
```


## Style Guide

### General

- **Prefer `yarn`** over `npm`.
- **Use tabs**, not spaces.
- **Quotation marks**:
  - Use double quotes for *text*, as in, human readable text.
    > ```typescript
    > const message = "Uh oh, something went wrong!";
    > const title = "The title of my page!";
  - Use single quotes `'foo'` for things that function as "keys".
    > ```typescript
    > const foo: Foo = { type: 'bar' };
    > myArray['data'];
    > <div className={ foo ? 'bar' : 'baz' }>
    > import Foo from 'foo';
  - Use of double quotations for JSX element props is acceptable.
    > ```tsx
    > <div className="foo bar baz">
    > ```
- **Inline braces should be spaced.**
  > ```typescript
  > const foo = { baz: 'bar' };
  > // NOT {baz: 'bar'}
- **JSX element props should *not* be spaced** around the equals sign.
  > ```tsx
  > <input type="text" />
  > ```

### Imports

- **Always use aliases** when possibe.
  > ```typescript
  > import Example  from '@components/Example';
  > ```

- **Always use the most specific alias possible**.
  > ```typescript
  > import Button from '@UI/Button';
  > // NOT from '@components/UI/Button'
  > ```

### Typing

- **Always strictly type** when possible.
  This includes:
  Callback function parameters
  > ```typescript
  > .then((res: ResponseType) => {
  >     ...
  > })
  Component definitions:
  > ```typescript
  > const Example: React.FC<ExampleProps> = ({ ... }) => { ... };
  Callback types:
  > ```typescript
  > type MyCallback = (value: string): void;
  > function myFunction(cb: MyCallback) { ... }
  HTML component props:
  > ```tsx
  > const type: ButtonHTMLAttributes<HTMLButtonElement>['type'] = 'submit';
  > <button type={ type }>
  > ```

- **The `children` prop**:
  - Should always be optional (`children?`), unless specifically required.
  - Should be of type `React.ReactNode` unless something more specific is required.
  > ```typescript
  > interface MyProps: {
  >     children?: React.ReactNode,
  > }
  > // ...
  > <div>
  >     { children }
  > </div>
  >```

- **Void return signatures** do not need to be typed.
  
- **Define and export types where they naturally belong.**
  This may be in a component, or an API wrapper, or anything else. Avoid having a `custom-types.d.ts` or any related documents.

### Components

- **Use only functional components** where possible.
  An example component should look like this:
  > ```tsx
  > // src/components/Example/index.tsx
  >
  > import React from "react";
  > 
  > interface TestProps {
  > 	foo: string,
  > 	bar?: number,
  > }
  > 
  > export const Test: React.FC<TestProps> = ({
  > 	foo,
  > 	bar = 42,
  > }) => {
  > 	return (
  > 	<>
  > 		<h1>{ foo }</h1>
  > 		<span>{ bar }</span>
  > 	</>
  > 	);
  > };
  > 
  > export default Test;
  > ```

- **Properly type component props.**

### Component Folder Structure

- Components should reside within their own folders.
  > A `Test` component should reside in `src/components/Test/`.

- The component's `index.tsx` should `export default` the namesake component.
  > The file `src/components/Test/index.tsx` should `export default Test;`.

- A component's `index.tsx` may also choose to export related components or types.
- A component's folder may include supporting assets such as `test.css`.
  Avoid assets such as icons, unless they are truly component-specific.
- A component's custom hooks should exist in a `hooks.ts` file in the component folder.
- Components should not define any API-type code.
