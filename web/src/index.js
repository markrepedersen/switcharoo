import React from "react";
import ReactDOM from "react-dom";
import {Admin, Resource} from "react-admin";

import {UsersList} from "./components/users/show";
import {UserEdit} from "./components/users/edit";
import {UserAdd} from "./components/users/add";

import {FeatureList} from "./components/features/show";
import {FeatureEdit} from "./components/features/edit";
import {FeatureAdd} from "./components/features/add";

import {PermissionList} from "./components/permissions/show";
import {PermissionEdit} from "./components/permissions/edit";
import {PermissionAdd} from "./components/permissions/add";

import {Home} from "./components/home";

import {authProvider, dataProvider} from "./utils/providers";

import FeaturesIcon from "@material-ui/icons/ToggleOn";
import UsersIcon from "@material-ui/icons/Group";
import {createMuiTheme} from "@material-ui/core/styles";

const theme = createMuiTheme({
  palette: {
    type: "dark", // Switching the dark mode on is a single property value change.
  },
});

ReactDOM.render(
  <Admin
    theme={theme}
    dashboard={Home}
    authProvider={authProvider}
    dataProvider={dataProvider}
  >
    <Resource
      name="users"
      list={UsersList}
      edit={UserEdit}
      create={UserAdd}
      icon={UsersIcon}
    />
    <Resource
      name="permissions"
      list={PermissionList}
      edit={PermissionEdit}
      create={PermissionAdd}
    />
    <Resource
      name="features"
      list={FeatureList}
      edit={FeatureEdit}
      create={FeatureAdd}
      icon={FeaturesIcon}
    />
  </Admin>,
  document.getElementById("root")
);
