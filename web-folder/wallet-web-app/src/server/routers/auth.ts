import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const authRouter = router({
  login: procedure
    .input(
      z.object({
        username: z.string(),
        password: z.string(),
      }),
    )
    .query(async (opts) => {
      const url = "http://localhost:8080/api/login"
      let login = await axios.request({
        url,
        method: "POST",
        data: {
          username: `${opts.input.username}`,
          password: `${opts.input.password}`,
        },
      });

      return {
        login: login.data,
      };
    }),
  logoff: procedure
    .query(async (opts) => {
      const url = "http://localhost:8080/api/logoff"
      let logoff = await axios.request({
        url,
        method: "POST",
        data: {
          logoff: true
        },
      });

      return {
        logoff: logoff.data,
      };
    }),
});
