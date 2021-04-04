import React, {Component} from "react";
import Toggle from "../Toggle";
import SearchBar from "../SearchBar";
import Table from "react-bootstrap/Table";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import Col from "react-bootstrap/Col";
import Row from "react-bootstrap/Row";
import Modal from "react-bootstrap/Modal";
import axios from "axios";

export default class Dashboard extends Component {
  constructor(props) {
    super(props);
    this.state = {data: [], showAddFeatureModal: false};
  }

  async componentDidMount() {
    this.setState({data: (await this.getFeatures()) || []});
  }

  async addFeature() {
    const response = await axios.post("/api/features", {});
  }

  async getFeatures() {
    const response = await axios.get("/api/features");
    return response.data;
  }

  renderTableData() {
    return this.state.data.map((toggle) => {
      return (
        <tr>
          <td>{toggle.key}</td>
          <td>
            <Toggle small name={toggle.key} start={toggle.value} />
          </td>
        </tr>
      );
    });
  }

  renderTableHeader() {
    return [<th> </th>, <th> Name</th>, <th> Checkbox </th>];
  }

  render() {
    return (
      <>
        <SearchBar />
        <Button
          block
          size="sm"
          variant="primary"
          onClick={() => this.setState({showAddFeatureModal: true})}
        >
          Add
        </Button>
        <Modal
          show={this.state.showAddFeatureModal}
          onHide={() => this.setState({showAddFeatureModal: false})}
          backdrop="static"
          size="lg"
          keyboard={false}
          centered
        >
          <Modal.Header closeButton>
            <Modal.Title>Add Feature Toggle</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <Form>
              <Form.Group as={Row} controlId="name">
                <Form.Label column sm={2}>
                  Name
                </Form.Label>
                <Col sm={10}>
                  <Form.Control placeholder="Name" />
                </Col>
              </Form.Group>

              <Form.Group as={Row} controlId="description">
                <Form.Label column sm={2}>
                  Description
                </Form.Label>
                <Col sm={10}>
                  <Form.Control placeholder="Description" />
                </Col>
              </Form.Group>
              <fieldset>
                <Form.Group as={Row}>
                  <Form.Label as="legend" column sm={2}>
                    Initial Value
                  </Form.Label>
                  <Col sm={10}>
                    <Form.Check
                      aria-label="value-switch"
                      type="switch"
                      id="value-switch"
                    />
                  </Col>
                </Form.Group>
              </fieldset>
              <Form.Group as={Row}>
                <Col sm={{span: 10, offset: 2}}>
                  <Button type="submit">Save</Button>
                </Col>
              </Form.Group>
            </Form>
          </Modal.Body>
        </Modal>
        <Table striped bordered hover size="sm" variant="dark">
          <thead>
            <tr>{this.renderTableHeader()}</tr>
          </thead>
          <tbody>{this.renderTableData()}</tbody>
        </Table>
      </>
    );
  }
}
