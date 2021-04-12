import axios from "../utils/axios";

const API_PATH = "api";

function uri(resource, params) {
  return params && params.id
    ? `/${API_PATH}/${resource}/${params.id}`
    : `/${API_PATH}/${resource}`;
}

export const dataProvider = {
  getOne: async function (resource, params) {
    return axios.get(uri(resource, params));
  },

  getList: async function (resource, params) {
    const response = await axios.get(uri(resource, params));

    return {
      data: response.data,
      total: response.data.length,
    };
  },

  update: async function (resource, params) {
    return axios.put(uri(resource, params), params.data);
  },

  create: async function (resource, params) {
    return axios.post(uri(resource, params), params.data);
  },

  delete: async function (resource, params) {
    return axios.delete(uri(resource, params), params.data);
  },
};

export const authProvider = {
  login: async ({username, password}) => {
    const response = await axios.post(uri("login"), {
      email: username,
      password,
    });

    if (response.status >= 200 || response.status < 300) {
      localStorage.setItem("token", response.data.token);

      return Promise.resolve();
    }

    return Promise.reject();
  },
  logout: () => {
    localStorage.removeItem("token");
    return Promise.resolve();
  },
  checkError: ({status}) => {
    if (status === 401 || status === 403) {
      localStorage.removeItem("token");
      return Promise.reject();
    }
    return Promise.resolve();
  },
  checkAuth: () => {
    return localStorage.getItem("token") ? Promise.resolve() : Promise.reject();
  },
  getPermissions: () => Promise.resolve(),
};
