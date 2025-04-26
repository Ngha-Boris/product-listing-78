import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { ArrowLeft, Edit, Trash2 } from "lucide-react";
import { useToast } from "./ui/use-toast";

interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  image_url: string;
  is_draft: boolean;
  is_verified: boolean;
  created_at: string;
  category?: {
    id: string;
    name: string;
  };
  tags?: Array<{
    id: string;
    name: string;
  }>;
}

const VendorProducts: React.FC = () => {
  const navigate = useNavigate();
  const { toast } = useToast();
  const [products, setProducts] = useState<Product[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        const response = await fetch('http://localhost:8080/api/products');
        if (!response.ok) {
          throw new Error('Failed to fetch products');
        }
        const data = await response.json();
        setProducts(data);
      } catch (error) {
        console.error('Error fetching products:', error);
        toast({
          title: "Error loading products",
          description: "Failed to load products. Please try again.",
          variant: "destructive",
        });
      } finally {
        setIsLoading(false);
      }
    };

    fetchProducts();
  }, [toast]);

  const handleEdit = (productId: string) => {
    navigate(`/products/${productId}/edit`);
  };

  const handleDelete = async (productId: string) => {
    try {
      const response = await fetch(`http://localhost:8080/api/products/${productId}`, {
        method: 'DELETE',
      });

      if (!response.ok) {
        throw new Error('Failed to delete product');
      }

      setProducts(products.filter(product => product.id !== productId));
      toast({
        title: "Product deleted",
        description: "The product has been successfully deleted.",
      });
    } catch (error) {
      console.error('Error deleting product:', error);
      toast({
        title: "Error deleting product",
        description: "Failed to delete the product. Please try again.",
        variant: "destructive",
      });
    }
  };

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center items-center h-64">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-cm-green"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 cm-pattern-bg">
      <div className="container mx-auto px-4 py-8">
        <div className="flex items-center justify-between mb-8">
          <div className="flex items-center space-x-4">
            <Button
              variant="outline"
              size="icon"
              onClick={() => navigate(-1)}
              className="rounded-full"
            >
              <ArrowLeft className="h-4 w-4" />
            </Button>
            <div>
              <h1 className="text-3xl font-bold text-gray-800">All Products</h1>
              <p className="text-gray-600">Browse and manage products</p>
            </div>
          </div>
          <Button
            className="bg-cm-green hover:bg-cm-forest"
            onClick={() => navigate('/')}
          >
            Add New Product
          </Button>
        </div>

        {products.length === 0 ? (
          <div className="text-center py-12">
            <h2 className="text-2xl font-semibold text-gray-700 mb-2">No Products Yet</h2>
            <p className="text-gray-600 mb-6">Start by adding your first product</p>
            <Button
              className="bg-cm-green hover:bg-cm-forest"
              onClick={() => navigate('/')}
            >
              Add Your First Product
            </Button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {products.map((product) => (
              <Card key={product.id} className="overflow-hidden">
                <div className="aspect-square relative">
                  <img
                    src={product.image_url}
                    alt={product.name}
                    className="w-full h-full object-cover"
                  />
                  <div className="absolute top-2 right-2">
                    <Badge
                      variant={product.is_verified ? "default" : "secondary"}
                      className={product.is_verified ? "bg-cm-green" : "bg-gray-500"}
                    >
                      {product.is_verified ? "Verified" : "Pending"}
                    </Badge>
                  </div>
                </div>
                <CardHeader>
                  <CardTitle className="text-xl">{product.name}</CardTitle>
                  <CardDescription className="line-clamp-2">
                    {product.description}
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex justify-between items-center mb-4">
                    <span className="text-2xl font-bold text-cm-green">
                      {product.price.toLocaleString()} FCFA
                    </span>
                    {product.is_draft && (
                      <Badge variant="outline">Draft</Badge>
                    )}
                  </div>
                  {product.category && (
                    <div className="mb-2">
                      <Badge variant="outline" className="bg-cm-sand bg-opacity-30">
                        {product.category.name}
                      </Badge>
                    </div>
                  )}
                  {product.tags && product.tags.length > 0 && (
                    <div className="flex flex-wrap gap-1">
                      {product.tags.map((tag) => (
                        <Badge key={tag.id} variant="secondary" className="bg-muted">
                          {tag.name}
                        </Badge>
                      ))}
                    </div>
                  )}
                </CardContent>
                <CardFooter className="flex justify-between">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleEdit(product.id)}
                  >
                    <Edit className="h-4 w-4 mr-2" />
                    Edit
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    className="text-red-600 hover:text-red-700 hover:bg-red-50"
                    onClick={() => handleDelete(product.id)}
                  >
                    <Trash2 className="h-4 w-4 mr-2" />
                    Delete
                  </Button>
                </CardFooter>
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default VendorProducts;