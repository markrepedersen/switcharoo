import React, {Component} from "react";
import Toggle from "../Toggle";
import SearchBar from "../SearchBar";
import axios from "axios";

import "./index.css";

export default class Table extends Component {
  constructor(props) {
    super(props);
    this.state = {data: []};
  }

  async componentDidMount() {
    const response = await axios.get("/api/features");
    this.setState({data: response.data});
  }

  renderTableData() {
    return this.state.data.map((toggle, index) => {
      return (
        <tr key={index}>
          <td>{toggle.key}</td>
          <td>
            <Toggle small name={toggle.key} start={toggle.value} />
          </td>
        </tr>
      );
    });
  }

  renderTableHeader() {
    return [<th> Name</th>, <th> Checkbox </th>];
  }

  render() {
    return (
      <div>
        <h1 id="title">Feature Toggles</h1>
        <SearchBar />
        <table id="table">
          <tbody>
            <tr>{this.renderTableHeader()}</tr>
            {this.renderTableData()}
          </tbody>
        </table>
      </div>
    );
  }
}
