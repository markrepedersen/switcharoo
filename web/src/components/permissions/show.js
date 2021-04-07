import * as React from "react";
import {
  Filter,
  TextField,
  EditButton,
  Datagrid,
  TextInput,
  List,
} from "react-admin";

const PermissionFilter = (props) => (
  <Filter {...props}>
    <TextInput label="Search" source="q" alwaysOn />
  </Filter>
);

export const PermissionList = (props) => (
  <List filters={<PermissionFilter />} {...props}>
    <Datagrid rowClick="edit">
      <TextField source="id" />
      <TextField source="name" />
      <EditButton />
    </Datagrid>
  </List>
);
