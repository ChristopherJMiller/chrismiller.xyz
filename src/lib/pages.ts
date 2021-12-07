import React from 'react';


export interface Page {
  name: string,
  path: string,
  element: React.LazyExoticComponent<() => JSX.Element>

}

export const pages: Page[] = [
  { name: "Home", path: "/", element: React.lazy(() => import('../pages/Home')) }
]
