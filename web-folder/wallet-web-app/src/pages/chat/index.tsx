import { useEffect, useState } from 'react'
import { Button, Text, Grid, Textarea, Affix, rem } from '@mantine/core'
import { type ChatGPTMessage, ChatLine, LoadingChatLine } from './ChatLine'
import { useCookies } from 'react-cookie'
import { IconSend } from '@tabler/icons-react'
const COOKIE_NAME = 'manta-ai-chat-gpt3'

// default first message to display in UI (not necessary to define the prompt)
export const initialMessages: ChatGPTMessage[] = [
  {
    role: 'assistant',
    content: 'Hi! I am a friendly AI assistant. Ask me anything!',
  },
]


const InputMessage = ({ input, setInput, sendMessage }: any) => (
<Affix position={{bottom: rem(20), right: 0, left: rem(400)}}>
  <Grid align="center" justify="center">
    <Grid.Col span={10}>
      <Textarea
        aria-label="chat input"
        autosize
        withAsterisk
        minRows={1}
        value={input}
        // onKeyDown={(e) => {
        //   if (e.key === 'Enter') {
        //     sendMessage(input)
        //     setInput('')
        //   }
        // }}
        onChange={(e) => {
          setInput(e.target.value)
        }}
      />
    </Grid.Col>
    <Grid.Col span={2}>
      <Button
        onClick={() => {
          sendMessage(input)
          setInput('')
        }}
      >
        <IconSend />
      </Button>
    </Grid.Col>
  </Grid></Affix>
)
const Chats = () => {
  const [messages, setMessages] = useState<ChatGPTMessage[]>(initialMessages)
  const [input, setInput] = useState('')
  const [loading, setLoading] = useState(false)
  const [cookie, setCookie] = useCookies([COOKIE_NAME])

  useEffect(() => {
    if (!cookie[COOKIE_NAME]) {
      // generate a semi random short id
      const randomId = Math.random().toString(36).substring(7)
      setCookie(COOKIE_NAME, randomId)
    }
  }, [cookie, setCookie])

  // send message to API /api/chat endpoint
  const sendMessage = async (message: string) => {
    setLoading(true)
    const newMessages = [
      ...messages,
      { role: 'user', content: message } as ChatGPTMessage,
    ]
    setMessages(newMessages)
    const last10messages = newMessages.slice(-10) // remember last 10 messages

    const response = await fetch('/api/chat', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        messages: last10messages,
        user: cookie[COOKIE_NAME],
      }),
    })

    console.log('Edge function returned.')

    if (!response.ok) {
      throw new Error(response.statusText)
    }

    // This data is a ReadableStream
    const data = response.body
    if (!data) {
      return
    }

    const reader = data.getReader()
    const decoder = new TextDecoder()
    let done = false

    let lastMessage = ''

    while (!done) {
      const { value, done: doneReading } = await reader.read()
      done = doneReading
      const chunkValue = decoder.decode(value)

      lastMessage = lastMessage + chunkValue

      setMessages([
        ...newMessages,
        { role: 'assistant', content: lastMessage } as ChatGPTMessage,
      ])

      setLoading(false)
    }
  }

  return (
    <div>
      {messages.map(({ content, role }, index) => (
        <ChatLine key={index} role={role} content={content} />
      ))}

      {loading && <LoadingChatLine />}

      {messages.length < 2 && (
        <Text c="dimmed" fs="italic" fz="sm" ta="center">
          Type a message to start the conversation:
        </Text>
      )}
      <InputMessage
        input={input}
        setInput={setInput}
        sendMessage={sendMessage}
      />
    </div>
  )
}
export default Chats;

