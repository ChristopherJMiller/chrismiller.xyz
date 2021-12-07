import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { pages, Page } from '../lib/pages';

//@ts-ignore
import meImg from '../img/me.jpg?sizes[]=100&sizes[]=300&sizes[]=500&format=webp';

export default () => {
  const location = useLocation();
  const navbarItems = pages.map((page: Page) => (<div className={location.pathname == page.path ? "col selected" : "col"}><Link to={page.path}>{page.name}</Link></div>));
  return (
    <nav>
      <picture>
        <source srcSet={meImg.srcSet} type='image/webp' />
        <img alt="Photo of Me" src={meImg.src} srcSet={meImg.srcSet} className="rounded center" />
      </picture>
      <div className="row mt3">
        {navbarItems}
      </div>
    </nav>
  )
}
