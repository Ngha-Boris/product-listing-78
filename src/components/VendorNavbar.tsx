import React from "react";
import { Link } from "react-router-dom";
import { User } from "lucide-react";
import { Button } from "./ui/button";

const VendorNavbar: React.FC = () => {
  return (
    <nav className="bg-cm-green text-white px-4 py-3 shadow-md">
      <div className="container mx-auto flex justify-between items-center">
        <div className="flex items-center space-x-2">
          <Link to="/" className="font-bold text-xl flex items-center">
            <img
              src="/placeholder.svg"
              alt="Made in Cameroon"
              className="h-8 w-8 mr-2"
            />
            <span>Made in Cameroon</span>
          </Link>
        </div>
        <div className="flex items-center space-x-4">
          <Button variant="ghost" className="text-white hover:text-cm-yellow">
            Dashboard
          </Button>
          <Button variant="ghost" className="text-white hover:text-cm-yellow">
            My Products
          </Button>
          <Button variant="ghost" className="text-white hover:text-cm-yellow">
            Orders
          </Button>
          <Link to="/notifications">
            <Button variant="ghost" className="text-white hover:text-cm-yellow">
              Notifications
            </Button>
          </Link>
          <Button className="bg-white text-cm-green hover:bg-cm-sand">
            <User size={18} className="mr-2" />
            Account
          </Button>
        </div>
      </div>
    </nav>
  );
};

export default VendorNavbar;
