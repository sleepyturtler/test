// Simple Discord bot (Node.js). Single file, no database.
// Setup: npm install   |   Run: DISCORD_TOKEN=xxx npm start
// Requires discord.js 14.19 or newer for Components V2.

const {
  Client,
  Events,
  GatewayIntentBits,
  ContainerBuilder,
  TextDisplayBuilder,
  SeparatorBuilder,
  MessageFlags,
} = require('discord.js');

const REACT_WORD = 'pizza'; // reacts with a pizza emoji
const REACT_EMOJI = '🍕';
const EMBED_WORD = 'embed'; // replies with a Components V2 container

// Runs an async action and logs how long it took, even if it fails.
async function timed(label, fn) {
  const start = performance.now();
  try {
    return await fn();
  } finally {
    console.log(`[timing] ${label} took ${(performance.now() - start).toFixed(1)} ms`);
  }
}

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent, // enable in the developer portal too
  ],
});

client.once(Events.ClientReady, (c) => {
  console.log(`Logged in as ${c.user.tag}`);
});

client.on(Events.MessageCreate, async (message) => {
  if (message.author.bot) return;

  const text = message.content.toLowerCase();

  // Basic commands
  switch (text.trim()) {
    case '!ping':
      return void (await timed('!ping reply', () => message.reply('Pong!')).catch(console.error));
    case '!hello':
      return void (await timed('!hello reply', () =>
        message.reply(`Hello, ${message.author.username}!`)
      ).catch(console.error));
    case '!help':
      return void (await timed('!help reply', () =>
        message.reply(
          `Commands: \`!ping\`, \`!hello\`, \`!help\`\n` +
            `Say "${REACT_WORD}" and I react. Say "${EMBED_WORD}" and I send a Components V2 message.`
        )
      ).catch(console.error));
  }

  // React if the word is in the message
  if (text.includes(REACT_WORD)) {
    await timed('reaction', () => message.react(REACT_EMOJI)).catch(console.error);
  }

  // Components V2 message if the word is in the message
  if (text.includes(EMBED_WORD)) {
    const container = new ContainerBuilder()
      .setAccentColor(0x5865f2)
      .addTextDisplayComponents(
        new TextDisplayBuilder().setContent('## Components V2')
      )
      .addSeparatorComponents(new SeparatorBuilder())
      .addTextDisplayComponents(
        new TextDisplayBuilder().setContent(
          `You said the magic word, **${message.author.username}**.`
        )
      );

    await timed('components v2 send', () =>
      message.reply({ components: [container], flags: MessageFlags.IsComponentsV2 })
    ).catch(console.error);
  }
});

client.login(process.env.DISCORD_TOKEN);
