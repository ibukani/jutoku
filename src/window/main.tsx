import React from "react";
import ReactDOM from "react-dom/client";
import { Clock } from "../feature/clock/Clock";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Clock />
  </React.StrictMode>
);
