import React, { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Badge } from "../components/ui/badge";

interface Notification {
  id: string;
  message: string;
  is_read: boolean;
  created_at: string;
}

const VENDOR_ID = "00000000-0000-0000-0000-000000000000"; // Replace with real vendor ID if available

const VendorNotifications: React.FC = () => {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`http://localhost:8080/api/vendors/${VENDOR_ID}/notifications`)
      .then((res) => res.json())
      .then((data) => setNotifications(data))
      .catch(() => setNotifications([]))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="container mx-auto px-4 py-8 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle>Notifications</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div>Loading...</div>
          ) : notifications.length === 0 ? (
            <div className="text-muted-foreground">No notifications found.</div>
          ) : (
            <ul className="space-y-4">
              {notifications.map((n) => (
                <li key={n.id} className="border-b pb-2 last:border-b-0 last:pb-0">
                  <div className="flex items-center justify-between">
                    <span>{n.message}</span>
                    <Badge variant={n.is_read ? "secondary" : "default"}>
                      {n.is_read ? "Read" : "Unread"}
                    </Badge>
                  </div>
                  <div className="text-xs text-muted-foreground mt-1">
                    {new Date(n.created_at).toLocaleString()}
                  </div>
                </li>
              ))}
            </ul>
          )}
        </CardContent>
      </Card>
    </div>
  );
};

export default VendorNotifications; 