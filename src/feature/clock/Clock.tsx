import { useEffect, useState } from "react";
import "./Clock.scss";

export const Clock = () => {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="container">
      <h2>{time.toLocaleDateString()}</h2>
      <h1>{time.toLocaleTimeString()}</h1>
    </div>
  );
};
