import axios from 'axios';
import { procedure, router } from '../trpc';

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;

export const chatRouter = router({
  chat: procedure
    .query(async () => {
      try {
        const url = "https://api.openai.com/v1/chat/completions"
        const headers = {
          "Authorization": `Bearer ${OPENAI_API_KEY}`,
          "Content-Type": "application/json",
        }
        let response = await axios.request({
          method: "POST",
          url,
          data: {
            model: "gpt-3.5-turbo",
            messages: [
              {
                role: "system",
                content: "You are a helpful assistant."
              },
              {
                role: "user",
                content: "Hello!"
              }
            ]
          },
          headers
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
