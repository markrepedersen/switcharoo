import React, {Component} from "react";
import {BrowserRouter, Route, Switch} from "react-router-dom";
import Dashboard from "../Dashboard";
import Login from "../Login";
import Header from "../Header";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {isLogged: false};
  }

  async componentDidMount() {
    if (localStorage.getItem("token")) {
      this.setState({isLoggedIn: true});
    }
  }

  render() {
    if (!this.state.isLoggedIn) {
      return <Login />;
    }

    return (
      <div className="wrapper">
        <BrowserRouter>
          <Header />
          <Switch>
            <Route path="/">
              <Dashboard />
            </Route>
          </Switch>
        </BrowserRouter>
      </div>
    );
  }
}
