import assert from "assert";
import axios from "axios";

const url = "http://localhost:5000";

describe("Boilerplate", async function () {
  this.timeout(10000);

  let id = "";
  let tokens = "";

  it("Creates a new user ", async () => {
    const res = await axios.post(`${url}/api/user`, {
      name: "John Doe",
      email: "me@me.com",
      password: "password",
    });

    id = res.data.id;

    assert.strictEqual(res.status, 201);
  });

  it("Logs in a user", async () => {
    const res = await axios.post(
      `${url}/api/auth/login`,
      {
        email: "me@me.com",
        password: "password",
      },
      {
        withCredentials: true,
      }
    );

    let access_token = res.headers["set-cookie"][0].split(";")[0] + ";";
    let refresh_token = res.headers["set-cookie"][1].split(";")[0] + ";";

    tokens = `${access_token} ${refresh_token}`;

    assert.strictEqual(res.status, 200);
  });

  it("Gets a user by id", async () => {
    const res = await axios.get(`${url}/api/user/${id}`, {
      headers: {
        Cookie: tokens,
      },
    });

    assert.strictEqual(res.status, 200);
  });

  it("logs out a user", async () => {
    const res = await axios.get(`${url}/api/auth/logout`, {
      headers: {
        Cookie: tokens,
      },
    });

    assert.strictEqual(res.status, 204);
  });
});
