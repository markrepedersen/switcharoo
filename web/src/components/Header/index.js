import React from "react";
import NavigationBar from "react-bootstrap/Navbar";
import Nav from "react-bootstrap/Nav";
import {useHistory} from "react-router-dom";

import "./index.css";

export default function Header() {
  const history = useHistory();

  function handleLogout() {
    localStorage.removeItem("token");
    history.push("/login");
  }

  return (
    <NavigationBar>
      <Nav>
        <Nav.Item>
          <Nav.Link onClick={handleLogout}>Logout</Nav.Link>
        </Nav.Item>
      </Nav>
    </NavigationBar>
  );
}
