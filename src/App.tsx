import React from 'react';
import { Route, Routes } from 'react-router';
import Nav from './components/Nav';
import { pages, Page } from './lib/pages'

const routedPages = pages.map((page: Page) => 
  <Route key={page.name} path={page.path} element={
    <React.Suspense fallback={<>...</>}>
      <page.element />
    </React.Suspense>
  } 
  />
);

export default () => {
  return (
    <div className="c">
      <Nav />
      <Routes>
        {routedPages}
      </Routes>
    </div>
  )
}
