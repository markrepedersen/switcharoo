import React, {Component} from "react";

export class SearchBar extends Component {
  render() {
    return (
      <form>
        <input type="text" name="search" placeholder="Search Toggles" />
      </form>
    );
  }
}
