import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { pages, Page } from '../lib/pages';

export default () => {
  const location = useLocation();
  const navbarItems = pages.map((page: Page) => (<div className={location.pathname == page.path ? "row selected" : "row"}><Link to={page.path}>{page.name}</Link></div>));

  return (
    <nav>
      <div className="row">
        {navbarItems}
      </div>
    </nav>
  )
}
