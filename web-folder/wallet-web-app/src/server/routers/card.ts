import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const cardRouter = router({
  create: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
        cname: z.string(),
        cbalance: z.string(),
        ctype: z.string(),
        caccount: z.string(),
        cnumber: z.string(),
        cvv: z.string(),
        cvalid: z.string(),
        cdescription: z.string(),
      }),
    )
    .mutation(async (opts) => {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      let response = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id,
          method,
          params: {
            data: {
              cname: `${opts.input.cname}`,
              cbalance: `${opts.input.cbalance}`,
              ctype: `${opts.input.ctype}`,
              caccount: `${opts.input.caccount}`,
              cnumber: `${opts.input.cnumber}`,
              cvv: `${opts.input.cvv}`,
              cvalid: `${opts.input.cvalid}`,
              cdescription: `${opts.input.cdescription}`,
            }
          }
        }
      });

      return {
        response: response.data,
      };
    }),
  list: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
      }),
    )
    .query(async (opts) => {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      try {

        let data = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id,
            method,
          }
        });

        return {
          data: data.data,
        };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error",
          solution: "Login to Server or Check your internet connection"
        }

      }
    }),
});
