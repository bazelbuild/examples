/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  // https://nextjs.org/docs/messages/export-image-api
  images: {
    unoptimized: true,
  },
};

module.exports = nextConfig;
