import type { Metadata } from 'next';
import './globals.css';
import Providers from './providers';
import Sidebar from '@/components/Sidebar';

export const metadata: Metadata = {
  title: 'Gradient Barter Protocol',
  description: 'Warehouse-backed, token-assisted barter network for real-world items',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-gray-50 text-gray-900 antialiased">
        <Providers>
          <Sidebar />
          <div className="lg:pl-64">
            {children}
          </div>
        </Providers>
      </body>
    </html>
  );
}
