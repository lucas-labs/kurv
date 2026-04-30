export {};

declare global {
    interface Window {
        __SERVER__: {
            BASE_URL: string;
            VERSION: string;
        };
    }
}
