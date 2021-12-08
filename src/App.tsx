import React from 'react';
import { Route, Routes } from 'react-router';
import Nav from './components/Nav';
import { pages, Page } from './lib/pages'

const routedPages = pages.filter((p: Page) => p.path).map((page: Page) => 
  <Route key={page.name} path={page.path} element={
    <React.Suspense fallback={<>...</>}>
      {page.element ? <page.element /> : null}
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
