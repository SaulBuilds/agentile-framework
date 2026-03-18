export interface ShipmentParams {
  originAddress: Address;
  destinationAddress: Address;
  weight: number; // in ounces
  dimensions?: { length: number; width: number; height: number }; // inches
  carrier?: string;
  serviceLevel?: string;
  metadata?: Record<string, any>;
}

export interface Address {
  street: string;
  city: string;
  state: string;
  zip: string;
  country: string;
  name?: string;
  phone?: string;
}

export interface ShipmentResult {
  providerId: string;
  carrier: string;
  trackingNumber: string;
  labelUrl?: string;
  estimatedDelivery?: Date;
  cost: number; // in cents
}

export interface RateParams {
  originZip: string;
  destinationZip: string;
  weight: number; // ounces
  dimensions?: { length: number; width: number; height: number };
}

export interface Rate {
  provider: string;
  carrier: string;
  serviceLevel: string;
  cost: number; // cents
  estimatedDays: number;
  currency: string;
}

export interface ProviderTrackingEvent {
  eventType: string;
  timestamp: Date;
  location?: string;
  description?: string;
  rawData?: any;
}

export interface DeliveryProvider {
  readonly name: string;

  createShipment(params: ShipmentParams): Promise<ShipmentResult>;
  getTrackingEvents(trackingNumber: string): Promise<ProviderTrackingEvent[]>;
  cancelShipment(providerId: string): Promise<void>;
  getRates(params: RateParams): Promise<Rate[]>;
}
