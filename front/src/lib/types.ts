export type UserRole = "user" | "employee" | "admin";

export interface AuthUser {
  id: string;
  email: string;
  full_name: string;
  phone: string | null;
  wallet_balance: number;
  role: UserRole;
  is_active: boolean;
}

export interface CurrentUserResponse {
  user: AuthUser;
}

export interface LoginResponse {
  token: string;
  user: AuthUser;
}

export interface PaginationMeta {
  current_page: number;
  per_page: number;
  total_items: number;
  total_pages: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  meta: PaginationMeta;
}

export interface Category {
  id: string;
  name: string;
  description: string | null;
}

export interface UserAdmin {
  id: string;
  email: string;
  full_name: string;
  phone: string | null;
  wallet_balance: number;
  role: UserRole;
  is_active: boolean;
}

export interface ServiceItem {
  id: string;
  category_id: string;
  name: string;
  description: string | null;
  base_price: number;
  duration_minutes: number;
  is_active: boolean;
}

export interface ProductItem {
  id: string;
  category_id: string;
  sku: string;
  name: string;
  description: string | null;
  unit_price: number;
  stock_qty: number;
  is_active: boolean;
}

export interface OrderItem {
  item_id: string;
  item_type: "service" | "product";
  reference_id: string;
  title: string;
  quantity: number;
  unit_price: number;
}

export interface Order {
  id: string;
  order_number: string;
  user_id: string;
  current_status_code: string;
  contact_name: string;
  contact_phone: string;
  contact_email: string;
  delivery_address: string;
  total_amount: number;
  items: OrderItem[];
}

export interface OrderSummary {
  id: string;
  order_number: string;
  current_status_code: string;
  total_amount: number;
}

export interface AvailableSlot {
  scheduled_at: string;
  is_available: boolean;
}

export interface Appointment {
  id: string;
  order_id: string;
  employee_user_id: string | null;
  scheduled_at: string;
  location: string;
  appointment_status: string;
}

export interface Payment {
  id: string;
  order_id: string;
  user_id: string;
  payment_method: string;
  payment_status: string;
  amount: number;
  currency: string;
  comment: string | null;
  paid_at: string | null;
}

export interface PaymentCheckoutResponse {
  payment: Payment;
  wallet_balance_after: number;
}

export interface CatalogQuery {
  page?: number;
  per_page?: number;
  search?: string;
  category_id?: string;
  min_price?: number;
  max_price?: number;
  only_active?: boolean;
}

export interface RegisterPayload {
  email: string;
  password: string;
  full_name: string;
  phone?: string;
}

export interface ServiceCreatePayload {
  category_id: string;
  name: string;
  description?: string;
  base_price: number;
  duration_minutes: number;
}

export interface ProductCreatePayload {
  category_id: string;
  sku: string;
  name: string;
  description?: string;
  unit_price: number;
  stock_qty: number;
}

export interface CategoryCreatePayload {
  name: string;
  description?: string;
}

export interface UsersQuery {
  page?: number;
  per_page?: number;
  search?: string;
  role?: UserRole;
  is_active?: boolean;
}

export interface LoginPayload {
  email: string;
  password: string;
}

export interface CheckoutPayload {
  contact_name: string;
  contact_phone: string;
  contact_email: string;
  delivery_address: string;
}

export interface AppointmentPayload {
  scheduled_at: string;
  location: string;
}

export interface OrdersReport {
  total_orders: number;
  total_amount: number;
  draft_orders: number;
  awaiting_payment_orders: number;
  completed_orders: number;
}

export interface PaymentsReport {
  total_payments: number;
  paid_amount: number;
  failed_payments: number;
}
