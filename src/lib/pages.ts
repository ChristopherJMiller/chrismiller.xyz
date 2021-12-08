import React from 'react';


export interface Page {
  name: string,
  path?: string,
  disabled?: boolean,
  element?: React.LazyExoticComponent<() => JSX.Element>
  redirect?: string,
}

export const pages: Page[] = [
  { name: "Home", path: "/", element: React.lazy(() => import('../pages/Home')) },
  { name: "Blog", path: "/blog", disabled: true },
  { name: "Tech", path: "/tech", disabled: true },
  { name: "Art", path: "/art", disabled: true },
  { name: "Resume", redirect: "https://raw.githubusercontent.com/ChristopherJMiller/Resume/master/resume.pdf" },
]
