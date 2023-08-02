import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const contactRouter = router({
  list: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
      }),
    )
    .query(async (opts) => {
    try {
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
        }
      });

      return {
        response: response.data,
      };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
  create: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
        ref_id: z.string(),
        association: z.string(),
        username: z.string(),
        name: z.string(),
        email: z.string(),
      }),
    )
    .mutation(async (opts) => {
    try {
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
              ref_id: `${opts.input.ref_id}`,
              association: `${opts.input.association}`,
              username: `${opts.input.username}`,
              name: `${opts.input.name}`,
              email: `${opts.input.email}`,
            }
          }
        }
      });

      return {
        response: response.data,
      };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
});

