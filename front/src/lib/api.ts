import {
  Appointment,
  AppointmentPayload,
  AvailableSlot,
  CategoryCreatePayload,
  CatalogQuery,
  Category,
  CheckoutPayload,
  CurrentUserResponse,
  LoginPayload,
  LoginResponse,
  OrdersReport,
  Order,
  OrderSummary,
  PaginatedResponse,
  PaymentCheckoutResponse,
  PaymentsReport,
  ProductCreatePayload,
  ProductItem,
  RegisterPayload,
  ServiceCreatePayload,
  ServiceItem,
  UserAdmin,
  UsersQuery,
  UserRole,
} from "@/lib/types";

const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL?.replace(/\/$/, "") || "http://localhost:8080";

type QueryParamValue = string | number | boolean | undefined;
type QueryParams = Record<string, QueryParamValue>;

export class ApiError extends Error {
  status: number;

  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

function buildUrl(path: string, query?: QueryParams) {
  const url = new URL(path, API_BASE_URL);

  if (query) {
    for (const [key, value] of Object.entries(query)) {
      if (value !== undefined && value !== "") {
        url.searchParams.set(key, String(value));
      }
    }
  }

  return url.toString();
}

async function parseResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const message = await response.text();
    throw new ApiError(response.status, message || "API request failed");
  }

  if (response.status === 204) {
    return undefined as T;
  }

  const text = await response.text();
  if (!text) {
    return undefined as T;
  }

  return JSON.parse(text) as T;
}

async function request<T>(
  path: string,
  init?: RequestInit,
  token?: string | null,
  query?: QueryParams,
) {
  const headers = new Headers(init?.headers);

  if (!headers.has("Content-Type") && init?.body) {
    headers.set("Content-Type", "application/json");
  }

  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }

  const response = await fetch(buildUrl(path, query), {
    ...init,
    headers,
    cache: "no-store",
  });

  return parseResponse<T>(response);
}

export function getServices(query?: CatalogQuery) {
  return request<PaginatedResponse<ServiceItem>>(
    "/api/catalog/services",
    undefined,
    null,
    query ? { ...query } : undefined,
  );
}

export function getProducts(query?: CatalogQuery) {
  return request<PaginatedResponse<ProductItem>>(
    "/api/catalog/products",
    undefined,
    null,
    query ? { ...query } : undefined,
  );
}

export function getServiceCategories() {
  return request<Category[]>("/api/catalog/categories/services");
}

export function getProductCategories() {
  return request<Category[]>("/api/catalog/categories/products");
}

export function createServiceCategory(payload: CategoryCreatePayload, token: string) {
  return request<Category>(
    "/api/catalog/categories/services",
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function createProductCategory(payload: CategoryCreatePayload, token: string) {
  return request<Category>(
    "/api/catalog/categories/products",
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function createService(payload: ServiceCreatePayload, token: string) {
  return request<ServiceItem>(
    "/api/catalog/services",
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function createProduct(payload: ProductCreatePayload, token: string) {
  return request<ProductItem>(
    "/api/catalog/products",
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function register(payload: RegisterPayload) {
  return request<LoginResponse>("/api/auth/register", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export function login(payload: LoginPayload) {
  return request<LoginResponse>("/api/auth/login", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export function getCurrentUser(token: string) {
  return request<CurrentUserResponse>("/api/auth/me", undefined, token);
}

export function logout(token: string) {
  return request<void>("/api/auth/logout", { method: "POST" }, token);
}

export function getCart(token: string) {
  return request<Order>("/api/orders/cart", undefined, token);
}

export function listOrders(token: string) {
  return request<OrderSummary[]>("/api/orders", undefined, token);
}

export function getOrder(id: string, token: string) {
  return request<Order>(`/api/orders/${id}`, undefined, token);
}

export function addServiceToCart(serviceId: string, quantity: number, token: string) {
  return request<Order>(
    "/api/orders/cart/services",
    {
      method: "POST",
      body: JSON.stringify({ service_id: serviceId, quantity }),
    },
    token,
  );
}

export function addProductToCart(productId: string, quantity: number, token: string) {
  return request<Order>(
    "/api/orders/cart/products",
    {
      method: "POST",
      body: JSON.stringify({ product_id: productId, quantity }),
    },
    token,
  );
}

export function removeServiceFromCart(serviceId: string, token: string) {
  return request<Order>(`/api/orders/cart/services/${serviceId}`, { method: "DELETE" }, token);
}

export function removeProductFromCart(productId: string, token: string) {
  return request<Order>(`/api/orders/cart/products/${productId}`, { method: "DELETE" }, token);
}

export function checkout(payload: CheckoutPayload, token: string) {
  return request<Order>(
    "/api/orders/checkout",
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function createPayment(orderId: string, token: string, comment?: string) {
  return request<PaymentCheckoutResponse>(
    `/api/orders/${orderId}/payments`,
    {
      method: "POST",
      body: JSON.stringify({ comment: comment || null }),
    },
    token,
  );
}

export function getAvailableSlots(dateFrom: string, dateTo: string) {
  return request<AvailableSlot[]>(
    "/api/schedule/slots",
    undefined,
    null,
    { date_from: dateFrom, date_to: dateTo },
  );
}

export function createAppointment(orderId: string, payload: AppointmentPayload, token: string) {
  return request<Appointment>(
    `/api/orders/${orderId}/appointment`,
    {
      method: "POST",
      body: JSON.stringify(payload),
    },
    token,
  );
}

export function getUsers(token: string, query?: UsersQuery) {
  return request<PaginatedResponse<UserAdmin>>(
    "/api/users",
    undefined,
    token,
    query ? { ...query } : undefined,
  );
}

export function updateUserRole(userId: string, role: UserRole, token: string) {
  return request<UserAdmin>(
    `/api/users/${userId}/role`,
    {
      method: "PATCH",
      body: JSON.stringify({ role }),
    },
    token,
  );
}

export function updateUserActiveState(userId: string, isActive: boolean, token: string) {
  return request<UserAdmin>(
    `/api/users/${userId}/active-state`,
    {
      method: "PATCH",
      body: JSON.stringify({ is_active: isActive }),
    },
    token,
  );
}

export function getOrdersReport(dateFrom: string, dateTo: string, token: string) {
  return request<OrdersReport>(
    "/api/reports/orders",
    undefined,
    token,
    { date_from: dateFrom, date_to: dateTo },
  );
}

export function getPaymentsReport(dateFrom: string, dateTo: string, token: string) {
  return request<PaymentsReport>(
    "/api/reports/payments",
    undefined,
    token,
    { date_from: dateFrom, date_to: dateTo },
  );
}
