import React from "react";
import NavigationBar from "react-bootstrap/Navbar";
import Nav from "react-bootstrap/Nav";
import {useHistory} from "react-router-dom";
import axios from "axios";

import "./index.css";

export default function Header() {
  const history = useHistory();

  async function handleLogout() {
    try {
      await axios.post("/api/logout");
      await history.push("/");
    } catch (e) {
      console.log(e);
    }
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
