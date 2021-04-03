import React, {Component} from "react";
import {BrowserRouter, Route, Switch} from "react-router-dom";
import Form from "../Form";
import Login from "../Login";
import Header from "../Header";
import axios from "axios";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {id: null};
  }

  async componentDidMount() {
    let id = null;
    try {
      const response = await axios.get("/api/whoami");
      id = response.data.id;
    } catch (e) {
    } finally {
      await this.setState({id});
    }
  }

  render() {
    if (!this.state.id) {
      return <Login />;
    }

    return (
      <div className="wrapper">
        <BrowserRouter>
          <Header />
          <Switch>
            <Route path="/">
              <Form />
            </Route>
          </Switch>
        </BrowserRouter>
      </div>
    );
  }
}
