import React, {Component} from "react";
import _uniqueId from "lodash/uniqueId";
import axios from "axios";

import "./index.scss";

/**
 * Props:
 * - small: display labels or not
 * - disabled: component disabled or not
 * - name: name of the toggle
 * - start: initial value of checkbox
 */
export default class Toggle extends Component {
  constructor(props) {
    super(props);

    this.optionLabels = ["On", "Off"];
    this.state = {
      id: _uniqueId("feature-"),
      checked: this.props.start,
    };
  }

  async onChange(checked) {
    await axios.post("/api/features", {
      key: this.props.name,
      value: checked,
    });
    console.log(`set ${this.props.name} to ${checked}...`);
    await this.setState({checked: checked});
  }

  render() {
    return (
      <div
        className={"toggle-switch" + (this.props.small ? " small-switch" : "")}
      >
        <input
          id={this.state.id}
          type="checkbox"
          name={this.props.name}
          className="toggle-switch-checkbox"
          checked={this.state.checked}
          onChange={(e) => this.onChange(e.target.checked)}
          disabled={this.props.disabled}
        />
        <label className="toggle-switch-label" htmlFor={this.state.id}>
          <span
            className={
              this.props.disabled
                ? "toggle-switch-inner toggle-switch-disabled"
                : "toggle-switch-inner"
            }
            data-yes={this.optionLabels[0]}
            data-no={this.optionLabels[1]}
            tabIndex={-1}
          />
          <span
            className={
              this.props.disabled
                ? "toggle-switch-switch toggle-switch-disabled"
                : "toggle-switch-switch"
            }
          />
        </label>
      </div>
    );
  }
}
