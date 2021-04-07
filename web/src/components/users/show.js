import * as React from "react";
import {
  Filter,
  TextField,
  EmailField,
  EditButton,
  Datagrid,
  TextInput,
  List,
} from "react-admin";

const UsersFilter = (props) => (
  <Filter {...props}>
    <TextInput label="Search" source="q" alwaysOn />
  </Filter>
);

export const UsersList = (props) => (
  <List filters={<UsersFilter />} {...props}>
    <Datagrid rowClick="edit">
      <TextField source="id" />
      <EmailField source="email" type="email" />
      <EditButton />
    </Datagrid>
  </List>
);
