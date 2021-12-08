import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { pages, Page } from '../lib/pages';

//@ts-ignore
import meImg from '../img/me.jpg?sizes[]=100&sizes[]=300&sizes[]=500&format=webp';

const getLinkElement = (page: Page) => {
  if (page.redirect) {
    return <a href={page.redirect} target="_blank" rel="noreferrer">{page.name} <i className="bi-link-45deg"></i></a>
  } else if (page.disabled) {
    return <p className="disabled">{page.name}</p>
  } else if (page.path) {
    return <Link to={page.path}>{page.name}</Link>
  } else {
    null
  }
}

export default () => {
  const location = useLocation();
  const navbarItems = pages.map((page: Page) => (
    <div className={location.pathname == page.path ? "col selected" : "col"}>
      {getLinkElement(page)}
    </div>
    )
  );
  const locationName = pages.find((page: Page) => page.path === location.pathname);
  if (locationName) {
    document.title = `Chris M - ${locationName.name}`
  } else {
    document.title = 'Chris M - Page Not Found'
  }
  return (
    <nav>
      <picture>
        <source srcSet={meImg.srcSet} type='image/webp' sizes='(min-width: 1024px) 1024px, 100vw' />
        <img alt="Photo of Me" src={meImg.src} srcSet={meImg.srcSet} className="rounded center" loading="lazy" sizes='(min-width: 1024px) 1024px, 100vw' />
      </picture>
      <div className="row mt3">
        {navbarItems}
      </div>
    </nav>
  )
}
