import React, {Component} from "react";
import Toggle from "../Toggle";
import axios from "axios";

export default class Form extends Component {
  constructor(props) {
    super(props);
    this.state = {data: []};
  }

  async componentDidMount() {
    const response = await axios.get("/api/features");
    this.setState({data: response.data});
  }

  render() {
    const toggles = this.state.data.map((toggle) => (
      <Toggle name={toggle.key} start={toggle.value} />
    ));

    return <div>{toggles}</div>;
  }
}
