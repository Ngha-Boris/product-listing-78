import { Toaster } from "@/components/ui/toaster";
import { Toaster as Sonner } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Index from "./pages/Index";
import ProductPreview from "./pages/ProductPreview";
import NotFound from "./pages/NotFound";
import ProductForm from "./components/ProductForm";
import VendorProducts from "./components/VendorProducts";
import VendorNotifications from "./pages/VendorNotifications";

const queryClient = new QueryClient();

const App = () => (
  <QueryClientProvider client={queryClient}>
    <TooltipProvider>
      <Toaster />
      <Sonner />
      <Router>
        <Routes>
          <Route path="/" element={<Index />} />
          <Route path="/product-preview" element={<ProductPreview />} />
          <Route path="/products" element={<VendorProducts />} />
          <Route path="/products/:productId/edit" element={<ProductForm />} />
          <Route path="/notifications" element={<VendorNotifications />} />
          {/* ADD ALL CUSTOM ROUTES ABOVE THE CATCH-ALL "*" ROUTE */}
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Router>
    </TooltipProvider>
  </QueryClientProvider>
);

export default App;
