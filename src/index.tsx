import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";

import App from './App';

import "@ajusa/lit/dist/lit.css";
import "@ajusa/lit/dist/util.css";
import "./styles/index.scss"
import "bootstrap-icons/font/bootstrap-icons.scss";

ReactDOM.render(
  <BrowserRouter>
    <App />
  </BrowserRouter>,
  document.getElementById("root")
);
