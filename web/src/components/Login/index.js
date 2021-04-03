import React, {useState} from "react";
import Form from "react-bootstrap/Form";
import Button from "react-bootstrap/Button";
import axios from "axios";
import {useHistory} from "react-router-dom";

import "./index.css";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [incorrect, setIncorrect] = useState(false);
  const history = useHistory();

  function validateForm() {
    return email.length > 0 && password.length > 0;
  }

  async function handleSubmit(event) {
    event.preventDefault();
    try {
      await axios.post("/api/login", {
        email,
        password,
      });

      history.push("/");
    } catch (e) {
      setIncorrect(true);
    }
  }

  return (
    <div className="Login">
      <Form onSubmit={handleSubmit}>
        <Form.Group size="lg" controlId="username">
          <Form.Label>Email</Form.Label>
          <Form.Control
            autoFocus
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </Form.Group>
        <Form.Group size="lg" controlId="password">
          <Form.Label>Password</Form.Label>
          <Form.Control
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </Form.Group>
        <Button
          id="loginButton"
          block
          size="lg"
          type="submit"
          disabled={!validateForm()}
        >
          Login
        </Button>
        {incorrect && (
          <div className="message-box">
            <p>Incorrect credentials. Please try again.</p>
            <i className="fa fa-exclamation-circle" />
          </div>
        )}
      </Form>
    </div>
  );
}
