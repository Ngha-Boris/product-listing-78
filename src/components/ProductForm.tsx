import React, { useState, useEffect } from "react";
import { useToast } from "./ui/use-toast";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Textarea } from "./ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { Check, Image, Plus, Save } from "lucide-react";
import { Badge } from "./ui/badge";
import { useParams } from "react-router-dom";

interface Category {
  id: string;
  name: string;
}

interface Tag {
  id: string;
  name: string;
}

const ProductForm: React.FC = () => {
  const { productId } = useParams();
  const { toast } = useToast();
  const [activeTab, setActiveTab] = useState<string>("details");
  const [categories, setCategories] = useState<Category[]>([]);
  const [tags, setTags] = useState<Tag[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isProductLoading, setIsProductLoading] = useState(false);
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    price: "",
    stockQuantity: "",
    category: "",
    selectedTags: [] as string[],
    images: [] as File[],
    imagePreviewUrls: [] as string[],
    shippingInfo: "",
    dimensions: "",
    weight: "",
    materials: "",
    returnPolicy: "",
  });
  
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSavingDraft, setIsSavingDraft] = useState(false);
  const [isPreviewing, setIsPreviewing] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [categoriesResponse, tagsResponse] = await Promise.all([
          fetch('http://localhost:8080/api/categories'),
          fetch('http://localhost:8080/api/tags')
        ]);

        if (!categoriesResponse.ok || !tagsResponse.ok) {
          throw new Error('Failed to fetch data');
        }

        const categoriesData = await categoriesResponse.json();
        const tagsData = await tagsResponse.json();

        setCategories(categoriesData);
        setTags(tagsData);
      } catch (error) {
        console.error('Error fetching data:', error);
        toast({
          title: "Error loading data",
          description: "Failed to load categories and tags. Please try again.",
          variant: "destructive",
        });
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, [toast]);

  // Fetch product for editing
  useEffect(() => {
    if (!productId) return;
    setIsProductLoading(true);
    fetch(`http://localhost:8080/api/products`)
      .then(res => {
        if (!res.ok) throw new Error('Failed to fetch product');
        return res.json();
      })
      .then(products => {
        const product = products.find((p: { id: string }) => p.id === productId);
        if (product) {
          setFormData(prev => ({
            ...prev,
            name: product.name || "",
            description: product.description || "",
            price: product.price ? String(product.price) : "",
            stockQuantity: product.stockQuantity ? String(product.stockQuantity) : "",
            category: product.category_id || product.category || "",
            selectedTags: product.tags ? product.tags.map((t: { id: string }) => t.id) : [],
            images: [],
            imagePreviewUrls: product.image_url ? [product.image_url.startsWith('http') ? product.image_url : `http://localhost:8080${product.image_url}`] : [],
            shippingInfo: product.shippingInfo || "",
            dimensions: product.dimensions || "",
            weight: product.weight ? String(product.weight) : "",
            materials: product.materials || "",
            returnPolicy: product.returnPolicy || "",
          }));
        }
      })
      .catch(error => {
        toast({
          title: "Error loading product",
          description: "Failed to load product for editing.",
          variant: "destructive",
        });
      })
      .finally(() => setIsProductLoading(false));
  }, [productId, toast]);

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  const handleSelectChange = (name: string, value: string) => {
    setFormData({ ...formData, [name]: value });
  };

  const handleTagToggle = (tagId: string) => {
    setFormData((prev) => {
      const selectedTags = prev.selectedTags.includes(tagId)
        ? prev.selectedTags.filter((id) => id !== tagId)
        : [...prev.selectedTags, tagId];
      return { ...prev, selectedTags };
    });
  };

  const handleImageUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;
    const file = files[0];

    // Upload to backend
    const formDataUpload = new FormData();
    formDataUpload.append('file', file);
    try {
      const uploadRes = await fetch('http://localhost:8080/api/upload', {
        method: 'POST',
        body: formDataUpload,
      });
      if (!uploadRes.ok) throw new Error('Failed to upload image');
      const uploadData = await uploadRes.json();
      setFormData((prev) => ({
        ...prev,
        images: [file],
        imagePreviewUrls: [`http://localhost:8080${uploadData.image_url}`], // Use the correct property and absolute URL
      }));
      toast({
        title: 'Image uploaded',
        description: 'Your image has been uploaded successfully.',
      });
    } catch (error) {
      toast({
        title: 'Error uploading image',
        description: 'There was an error uploading your image. Please try again.',
        variant: 'destructive',
      });
    }
  };

  const removeImage = (index: number) => {
    setFormData((prev) => {
      const newImages = [...prev.images];
      const newPreviewUrls = [...prev.imagePreviewUrls];
      
      // Revoke the object URL to avoid memory leaks
      URL.revokeObjectURL(newPreviewUrls[index]);
      
      newImages.splice(index, 1);
      newPreviewUrls.splice(index, 1);
      
      return {
        ...prev,
        images: newImages,
        imagePreviewUrls: newPreviewUrls,
      };
    });
  };

  const saveDraft = async () => {
    setIsSavingDraft(true);
    
    try {
      // Simulate API call to backend
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      toast({
        title: "Draft saved",
        description: "Your product has been saved as a draft.",
      });
    } catch (error) {
      toast({
        title: "Error saving draft",
        description: "There was an error saving your draft. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsSavingDraft(false);
    }
  };

  const previewProduct = () => {
    setIsPreviewing(true);
    setActiveTab("preview");
  };

  const submitProduct = async () => {
    setIsSubmitting(true);
    
    try {
      // Prepare the product data
      const productData = {
        vendor_id: "00000000-0000-0000-0000-000000000000", // This should be the actual vendor ID from your auth system
        name: formData.name,
        description: formData.description,
        price: parseFloat(formData.price),
        image_url: formData.imagePreviewUrls[0] || "", // Using the first image as the main image
        is_draft: false,
        category_id: formData.category || null,
        tag_ids: formData.selectedTags.length > 0 ? formData.selectedTags : null
      };

      // Make the API call to create the product
      const response = await fetch('http://localhost:8080/api/products', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(productData),
      });

      if (!response.ok) {
        throw new Error('Failed to submit product');
      }

      const result = await response.json();
      
      toast({
        title: "Product submitted",
        description: "Your product has been submitted for verification.",
      });
      
      // Reset form data after successful submission
      setFormData({
        name: "",
        description: "",
        price: "",
        stockQuantity: "",
        category: "",
        selectedTags: [],
        images: [],
        imagePreviewUrls: [],
        shippingInfo: "",
        dimensions: "",
        weight: "",
        materials: "",
        returnPolicy: "",
      });
      
      setActiveTab("details");
    } catch (error) {
      console.error('Error submitting product:', error);
      toast({
        title: "Error submitting product",
        description: "There was an error submitting your product. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
      setIsPreviewing(false);
    }
  };

  const isFormValid = () => {
    return (
      formData.name &&
      formData.description &&
      formData.price &&
      formData.category &&
      formData.images.length > 0
    );
  };

  if (isProductLoading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-cm-green"></div>
        <span className="ml-4">Loading product...</span>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-6 max-w-4xl">
      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="details">Product Details</TabsTrigger>
          <TabsTrigger value="attributes">Attributes & Images</TabsTrigger>
          <TabsTrigger value="preview" disabled={!isFormValid() && !isPreviewing}>
            Preview
          </TabsTrigger>
        </TabsList>

        <TabsContent value="details">
          <Card>
            <CardHeader>
              <CardTitle>Basic Information</CardTitle>
              <CardDescription>
                Enter the basic details about your product.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="name">Product Name</Label>
                <Input
                  id="name"
                  name="name"
                  value={formData.name}
                  onChange={handleInputChange}
                  placeholder="Enter your product name"
                  required
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="description">Description</Label>
                <Textarea
                  id="description"
                  name="description"
                  value={formData.description}
                  onChange={handleInputChange}
                  placeholder="Describe your product in detail"
                  required
                  rows={5}
                />
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="price">Price (FCFA)</Label>
                  <Input
                    id="price"
                    name="price"
                    type="number"
                    value={formData.price}
                    onChange={handleInputChange}
                    placeholder="0"
                    required
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="stockQuantity">Stock Quantity</Label>
                  <Input
                    id="stockQuantity"
                    name="stockQuantity"
                    type="number"
                    value={formData.stockQuantity}
                    onChange={handleInputChange}
                    placeholder="0"
                  />
                </div>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="category">Category</Label>
                <Select 
                  value={formData.category} 
                  onValueChange={(value) => handleSelectChange("category", value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select a category" />
                  </SelectTrigger>
                  <SelectContent>
                    {categories.map((category) => (
                      <SelectItem key={category.id} value={category.id}>
                        {category.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </CardContent>
            <CardFooter className="flex justify-between">
              <Button variant="outline" onClick={saveDraft} disabled={isSavingDraft}>
                {isSavingDraft ? (
                  "Saving..."
                ) : (
                  <>
                    <Save size={16} className="mr-2" />
                    Save Draft
                  </>
                )}
              </Button>
              <Button onClick={() => setActiveTab("attributes")}>
                Next: Attributes & Images
              </Button>
            </CardFooter>
          </Card>
        </TabsContent>

        <TabsContent value="attributes">
          <Card>
            <CardHeader>
              <CardTitle>Product Attributes & Images</CardTitle>
              <CardDescription>
                Add specific details and images of your product.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>Product Tags</Label>
                <div className="flex flex-wrap gap-2">
                  {tags.map((tag) => (
                    <Badge
                      key={tag.id}
                      variant={formData.selectedTags.includes(tag.id) ? "default" : "outline"}
                      className={`cursor-pointer ${
                        formData.selectedTags.includes(tag.id)
                          ? "bg-cm-green hover:bg-cm-forest"
                          : ""
                      }`}
                      onClick={() => handleTagToggle(tag.id)}
                    >
                      {tag.name}
                      {formData.selectedTags.includes(tag.id) && (
                        <Check size={14} className="ml-1" />
                      )}
                    </Badge>
                  ))}
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="images">Product Images</Label>
                <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mt-2">
                  {formData.imagePreviewUrls.map((url, index) => (
                    <div 
                      key={index} 
                      className="relative aspect-square bg-muted rounded-md overflow-hidden group"
                    >
                      <img 
                        src={url} 
                        alt={`Product ${index + 1}`} 
                        className="w-full h-full object-cover"
                      />
                      <button
                        type="button"
                        onClick={() => removeImage(index)}
                        className="absolute inset-0 bg-black bg-opacity-50 opacity-0 group-hover:opacity-100 flex items-center justify-center text-white transition-opacity"
                      >
                        Remove
                      </button>
                    </div>
                  ))}
                  
                  {formData.imagePreviewUrls.length < 5 && (
                    <label 
                      htmlFor="image-upload" 
                      className="aspect-square border-2 border-dashed border-muted-foreground rounded-md flex flex-col items-center justify-center cursor-pointer hover:border-primary transition-colors"
                    >
                      <Image size={24} className="mb-2 text-muted-foreground" />
                      <span className="text-sm text-muted-foreground">Add Image</span>
                      <input
                        id="image-upload"
                        type="file"
                        accept="image/*"
                        onChange={handleImageUpload}
                        multiple
                        className="hidden"
                      />
                    </label>
                  )}
                </div>
                <p className="text-xs text-muted-foreground mt-1">
                  Upload up to 5 images. First image will be the cover. Max 2MB each (2G optimized).
                </p>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="materials">Materials</Label>
                  <Input
                    id="materials"
                    name="materials"
                    value={formData.materials}
                    onChange={handleInputChange}
                    placeholder="What is your product made from?"
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="dimensions">Dimensions</Label>
                  <Input
                    id="dimensions"
                    name="dimensions"
                    value={formData.dimensions}
                    onChange={handleInputChange}
                    placeholder="Size or dimensions (optional)"
                  />
                </div>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="weight">Weight (kg)</Label>
                  <Input
                    id="weight"
                    name="weight"
                    type="number"
                    value={formData.weight}
                    onChange={handleInputChange}
                    placeholder="0.00"
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="shippingInfo">Shipping Information</Label>
                  <Input
                    id="shippingInfo"
                    name="shippingInfo"
                    value={formData.shippingInfo}
                    onChange={handleInputChange}
                    placeholder="Shipping details (optional)"
                  />
                </div>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="returnPolicy">Return Policy</Label>
                <Textarea
                  id="returnPolicy"
                  name="returnPolicy"
                  value={formData.returnPolicy}
                  onChange={handleInputChange}
                  placeholder="Your return policy (optional)"
                  rows={3}
                />
              </div>
            </CardContent>
            <CardFooter className="flex justify-between">
              <Button variant="outline" onClick={() => setActiveTab("details")}>
                Back to Details
              </Button>
              <div className="flex space-x-2">
                <Button variant="outline" onClick={saveDraft} disabled={isSavingDraft}>
                  {isSavingDraft ? (
                    "Saving..."
                  ) : (
                    <>
                      <Save size={16} className="mr-2" />
                      Save Draft
                    </>
                  )}
                </Button>
                <Button 
                  onClick={previewProduct} 
                  disabled={!isFormValid()}
                >
                  Preview Product
                </Button>
              </div>
            </CardFooter>
          </Card>
        </TabsContent>

        <TabsContent value="preview">
          <Card>
            <CardHeader>
              <CardTitle>Product Preview</CardTitle>
              <CardDescription>
                Review how your product will appear to customers before submitting.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="mb-6 grid gap-6 md:grid-cols-2">
                <div className="bg-white rounded-lg overflow-hidden">
                  {formData.imagePreviewUrls.length > 0 ? (
                    <img
                      src={formData.imagePreviewUrls[0]}
                      alt={formData.name}
                      className="w-full h-64 object-cover"
                    />
                  ) : (
                    <div className="w-full h-64 bg-muted flex items-center justify-center">
                      <p className="text-muted-foreground">No image available</p>
                    </div>
                  )}
                  
                  {formData.imagePreviewUrls.length > 1 && (
                    <div className="flex mt-2 space-x-2 overflow-x-auto p-1">
                      {formData.imagePreviewUrls.slice(1).map((url, index) => (
                        <img
                          key={index}
                          src={url}
                          alt={`Product ${index + 2}`}
                          className="w-16 h-16 object-cover rounded"
                        />
                      ))}
                    </div>
                  )}
                </div>
                
                <div>
                  <h2 className="text-2xl font-bold mb-2">{formData.name || "Product Name"}</h2>
                  <div className="text-xl font-semibold text-cm-green mb-4">
                    {formData.price ? `${formData.price} FCFA` : "Price not set"}
                  </div>
                  
                  {formData.category && (
                    <div className="mb-2">
                      <Badge variant="outline" className="bg-cm-sand bg-opacity-30">
                        {categories.find(c => c.id === formData.category)?.name}
                      </Badge>
                    </div>
                  )}
                  
                  {formData.selectedTags.length > 0 && (
                    <div className="flex flex-wrap gap-1 mb-4">
                      {formData.selectedTags.map((tagId) => (
                        <Badge key={tagId} variant="secondary" className="bg-muted">
                          {tags.find(t => t.id === tagId)?.name}
                        </Badge>
                      ))}
                    </div>
                  )}
                  
                  <div className="text-sm mb-6">
                    {formData.stockQuantity ? (
                      <span className="text-cm-green">
                        In stock: {formData.stockQuantity} available
                      </span>
                    ) : (
                      <span className="text-muted-foreground">Stock quantity not set</span>
                    )}
                  </div>
                  
                  <div className="mb-6">
                    <h3 className="font-semibold mb-1">Description</h3>
                    <p className="text-sm text-gray-600">
                      {formData.description || "No description provided."}
                    </p>
                  </div>
                  
                  {(formData.materials || formData.dimensions || formData.weight) && (
                    <div className="mb-4">
                      <h3 className="font-semibold mb-1">Details</h3>
                      <ul className="text-sm space-y-1">
                        {formData.materials && (
                          <li><span className="font-medium">Materials:</span> {formData.materials}</li>
                        )}
                        {formData.dimensions && (
                          <li><span className="font-medium">Dimensions:</span> {formData.dimensions}</li>
                        )}
                        {formData.weight && (
                          <li><span className="font-medium">Weight:</span> {formData.weight} kg</li>
                        )}
                      </ul>
                    </div>
                  )}
                </div>
              </div>
              
              {(formData.shippingInfo || formData.returnPolicy) && (
                <div className="mt-6 grid gap-4 md:grid-cols-2">
                  {formData.shippingInfo && (
                    <div>
                      <h3 className="font-semibold mb-1">Shipping Information</h3>
                      <p className="text-sm text-gray-600">{formData.shippingInfo}</p>
                    </div>
                  )}
                  {formData.returnPolicy && (
                    <div>
                      <h3 className="font-semibold mb-1">Return Policy</h3>
                      <p className="text-sm text-gray-600">{formData.returnPolicy}</p>
                    </div>
                  )}
                </div>
              )}
            </CardContent>
            <CardFooter className="flex justify-between">
              <Button variant="outline" onClick={() => setActiveTab("attributes")}>
                Back to Edit
              </Button>
              <Button 
                onClick={submitProduct} 
                disabled={isSubmitting || !isFormValid()}
                className="bg-cm-green hover:bg-cm-forest"
              >
                {isSubmitting ? (
                  "Submitting..."
                ) : (
                  <>
                    <Plus size={16} className="mr-2" />
                    Submit Product
                  </>
                )}
              </Button>
            </CardFooter>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default ProductForm;
